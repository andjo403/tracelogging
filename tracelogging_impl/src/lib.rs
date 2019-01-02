#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro_hack::proc_macro_hack;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{
    parenthesized, parse_macro_input, Expr, ExprPath, Ident, LitByteStr, LitStr, Path, Token,
};

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_hack]
pub fn register(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RegisterStruct);
    let provider_name = args.provider_name;
    let bytes = provider_name.len() + 1;
    let guid_part1 = args.guid_part1;
    let guid_part2 = args.guid_part2;
    let guid_part3 = args.guid_part3;
    let guid_part4 = args.guid_part4;
    TokenStream::from(quote! {
        {
            let mut handle: winapi::shared::evntprov::REGHANDLE = 0;
            let guid = winapi::shared::guiddef::GUID {
                Data1: #guid_part1,
                Data2: #guid_part2,
                Data3: #guid_part3,
                Data4: [#(#guid_part4),*],
            };

            let mut result =
                unsafe { winapi::shared::evntprov::EventRegister(&guid, None, std::ptr::null_mut(), &mut handle) };

            if result == winapi::shared::winerror::ERROR_SUCCESS {
                #[repr(C, packed)]
                struct EventInformation {
                    size: u16,
                    data: [u8; #bytes],
                }

                let mut event_info = EventInformation {
                    size: std::mem::size_of::<EventInformation>() as u16,
                    data: [#(#provider_name),* , b'\0'],
                };

                unsafe {
                    result = winapi::shared::evntprov::EventSetInformation(
                        handle,
                        winapi::shared::evntprov::EventProviderSetTraits,
                        &event_info as *const _ as winapi::um::winnt::PVOID,
                        std::mem::size_of::<EventInformation>() as u32,
                    );
                }
                if result != winapi::shared::winerror::ERROR_SUCCESS {
                    println!("EventSetInformation failed with '{}'", result);
                }
            } else {
                println!("EventRegister failed with '{}'", result);
            }
            unsafe {
                trace_logging::HANDLE = Some(handle);
            }
        }
    })
}

#[proc_macro_hack]
pub fn write(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as WriteInput);
    let fields = args.fields.len() + 1;
    let event_meta_data = event_meta_data(&args);
    let result = TokenStream::from(quote! {
        {
            if let Some(handle) = unsafe { trace_logging::HANDLE } {
                #event_meta_data

                let event_descriptor = winapi::shared::evntprov::EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: 0,
                    Task: 0,
                    Keyword: 0,
                };

                unsafe {
                    winapi::shared::evntprov::EventWrite(
                        handle,
                        &event_descriptor,
                        #fields as u32,
                        event_data_descriptors.as_mut_ptr(),
                    )
                };
            }
        }
    });
    //println!("{}", result.to_string());
    result
}

#[proc_macro_hack]
pub fn write_start(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as WriteInput);
    let fields = args.fields.len() + 1;
    let event_meta_data = event_meta_data(&args);
    let result = TokenStream::from(quote! {
        {
            if let Some(handle) = unsafe { trace_logging::HANDLE } {
                #event_meta_data

                let event_descriptor = winapi::shared::evntprov::EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: 1, // start
                    Task: 0,
                    Keyword: 0,
                };

                trace_logging::GUID_STACK.with(|s| {
                    let mut stack = s.borrow_mut();
                    let mut current = winapi::um::cguid::GUID_NULL;
                    unsafe {
                        winapi::shared::evntprov::EventActivityIdControl(winapi::shared::evntprov::EVENT_ACTIVITY_CTRL_CREATE_ID,&mut current);
                    }
                    stack.push(current);

                    unsafe {
                        winapi::shared::evntprov::EventWriteTransfer(
                            handle,
                            &event_descriptor,
                            &current,
                            std::ptr::null(),
                            #fields as u32,
                            event_data_descriptors.as_mut_ptr(),
                        )
                    };
                });
            }
        }
    });
    //println!("{}", result.to_string());
    result
}

#[proc_macro_hack]
pub fn write_stop(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as WriteInput);
    let fields = args.fields.len() + 1;
    let event_meta_data = event_meta_data(&args);
    let result = TokenStream::from(quote! {
        {
            if let Some(handle) = unsafe { trace_logging::HANDLE } {
                #event_meta_data

                let event_descriptor = winapi::shared::evntprov::EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: 2, // stop
                    Task: 0,
                    Keyword: 0,
                };

                trace_logging::GUID_STACK.with(|s| {
                    let mut stack = s.borrow_mut();
                    let current = stack.pop().expect("write_start needs to done before write_stop");

                    unsafe {
                        winapi::shared::evntprov::EventWriteTransfer(
                            handle,
                            &event_descriptor,
                            &current,
                            std::ptr::null(),
                            #fields as u32,
                            event_data_descriptors.as_mut_ptr(),
                        )
                    };
                });
            }
        }
    });
    //println!("{}", result.to_string());
    result
}

#[proc_macro_hack]
pub fn write_tagged(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as WriteInput);
    let fields = args.fields.len() + 1;
    let event_meta_data = event_meta_data(&args);
    let result = TokenStream::from(quote! {
        {
            if let Some(handle) = unsafe { trace_logging::HANDLE } {
                #event_meta_data

                let event_descriptor = winapi::shared::evntprov::EVENT_DESCRIPTOR {
                    Id: 0,
                    Version: 0,
                    Channel: 0,
                    Level: 0,
                    Opcode: 0,
                    Task: 0,
                    Keyword: 0,
                };

                trace_logging::GUID_STACK.with(|s| {
                    let stack = s.borrow();
                    let current = stack.last().expect("write_start needs to done before write_stop");

                    unsafe {
                        winapi::shared::evntprov::EventWriteTransfer(
                            handle,
                            &event_descriptor,
                            current,
                            std::ptr::null(),
                            #fields as u32,
                            event_data_descriptors.as_mut_ptr(),
                        )
                    };
                });
            }
        }
    });
    //println!("{}", result.to_string());
    result
}

fn event_meta_data(args: &WriteInput) -> TokenStream2 {
    let event_meta_data_define = event_meta_data_define(args);
    let event_meta_data_init = event_meta_data_init(args);
    let fields = args.fields.len() + 1;

    let mut event_data_ref_fields = TokenStream2::new();
    for (i, field) in args.fields.iter().enumerate() {
        event_data_ref_fields.extend(event_data_ref_field(i, field));
    }

    let mut event_data_descriptors_fields = TokenStream2::new();
    for (i, field) in args.fields.iter().enumerate() {
        event_data_descriptors_fields.extend(event_data_descriptors_field(i, field));
    }

    quote! {
        #event_meta_data_define

        let event_meta_data = #event_meta_data_init

        #event_data_ref_fields

        let mut event_data_descriptors: [winapi::shared::evntprov::EVENT_DATA_DESCRIPTOR; #fields] = [
            winapi::shared::evntprov::EVENT_DATA_DESCRIPTOR {
                Ptr: &event_meta_data as *const _ as winapi::um::winnt::ULONGLONG,
                Size: std::mem::size_of::<EventMetaData>() as u32,
                u: unsafe { std::mem::zeroed() },
            },
            #event_data_descriptors_fields
        ];

        unsafe {
            event_data_descriptors[0].u.s_mut().Type =
                winapi::shared::evntprov::EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA;
        }
    }
}

fn event_meta_data_define(input: &WriteInput) -> TokenStream2 {
    let event_name_bytes = input.event_name.len() + 1;
    let mut fields = TokenStream2::new();
    for (i, field) in input.fields.iter().enumerate() {
        fields.extend(event_meta_data_field_define(i, field));
    }
    quote! {
        #[repr(C, packed)]
        struct EventMetaData {
            meta_size: u16,
            tags: u8,
            event_name: [u8; #event_name_bytes],
            #fields
        };
    }
}

fn event_meta_data_field_define(index: usize, input: &FieldInput) -> TokenStream2 {
    let name_bytes = input.field_name.len() + 1;
    let field_name = Ident::new(&format!("field_name_{}", index), Span::call_site());
    let in_type = Ident::new(&format!("in_type_{}", index), Span::call_site());
    quote! {
        #field_name: [u8; #name_bytes],
        #in_type: trace_logging::FieldType,
    }
}

fn event_meta_data_init(input: &WriteInput) -> TokenStream2 {
    let event_name = &input.event_name;
    let mut fields = TokenStream2::new();
    for (i, field) in input.fields.iter().enumerate() {
        fields.extend(event_meta_data_field_init(i, field));
    }
    quote! {
        EventMetaData {
            meta_size: std::mem::size_of::<EventMetaData>() as u16,
            tags: 0,
            event_name: [#(#event_name),* , b'\0'],
            #fields
        };
    }
}

fn event_meta_data_field_init(index: usize, input: &FieldInput) -> TokenStream2 {
    let event_name = &input.field_name;
    let field_type = &input.field_type;
    let field_name = Ident::new(&format!("field_name_{}", index), Span::call_site());
    let in_type = Ident::new(&format!("in_type_{}", index), Span::call_site());
    quote! {
        #field_name: [#(#event_name),*  , b'\0'],
        #in_type: #field_type,
    }
}

fn event_data_descriptors_field(index: usize, input: &FieldInput) -> TokenStream2 {
    let field_name = Ident::new(&format!("field_value_{}", index), Span::call_site());
    let ExprPath {
        path: Path { ref segments, .. },
        ..
    } = input.field_type;
    if segments
        .last()
        .map(|segment| segment.value().ident.to_string())
        == Some("ANSISTRING".to_string())
    {
        quote! {
            winapi::shared::evntprov::EVENT_DATA_DESCRIPTOR {
                Ptr: #field_name.as_ptr() as *const _ as winapi::um::winnt::ULONGLONG,
                Size: #field_name.len() as u32,
                u: unsafe { std::mem::zeroed() },
            },
        }
    } else {
        quote! {
            winapi::shared::evntprov::EVENT_DATA_DESCRIPTOR {
                Ptr: &#field_name as *const _ as winapi::um::winnt::ULONGLONG,
                Size: trace_logging::size_of(&#field_name),
                u: unsafe { std::mem::zeroed() },
            },
        }
    }
}

fn event_data_ref_field(index: usize, input: &FieldInput) -> TokenStream2 {
    let value = &input.value;
    let field_name = Ident::new(&format!("field_value_{}", index), Span::call_site());
    let ExprPath {
        path: Path { ref segments, .. },
        ..
    } = input.field_type;
    if segments
        .last()
        .map(|segment| segment.value().ident.to_string())
        == Some("ANSISTRING".to_string())
    {
        let field_name_store =
            Ident::new(&format!("field_value_store{}", index), Span::call_site());
        quote! {
            let #field_name_store = std::ffi::CString::new(#value).expect("CString::new failed");
            let #field_name = #field_name_store.as_bytes_with_nul();
        }
    } else {
        quote! {
            let #field_name = #value;
        }
    }
}

struct RegisterStruct {
    provider_name: Vec<u8>,
    guid_part1: u32,
    guid_part2: u16,
    guid_part3: u16,
    guid_part4: Vec<u8>,
}

impl Parse for RegisterStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let provider_name: LitByteStr = input.parse()?;
        let provider_name = provider_name.value();
        input.parse::<Token![,]>()?;
        let guid: LitStr = input.parse()?;
        let guid = guid.value();
        let guid_parts: Vec<&str> = guid.split('-').collect();
        if guid_parts.len() != 5 {
            return Err(input.error("guids shall contain 5 parts"));
        }
        if guid_parts[0].len() != 8 {
            return Err(input.error("guids part 1 shall contain 8 hexdigis"));
        }
        if guid_parts[1].len() != 4 {
            return Err(input.error("guids part 2 shall contain 4 hexdigis"));
        }
        if guid_parts[2].len() != 4 {
            return Err(input.error("guids part 3 shall contain 4 hexdigis"));
        }
        if guid_parts[3].len() != 4 {
            return Err(input.error("guids part 4 shall contain 4 hexdigis"));
        }
        if guid_parts[4].len() != 12 {
            return Err(input.error("guids part 5 shall contain 12 hexdigis"));
        }
        let guid_part1 = u32::from_str_radix(guid_parts[0], 16)
            .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?;
        let guid_part2 = u16::from_str_radix(guid_parts[1], 16)
            .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?;
        let guid_part3 = u16::from_str_radix(guid_parts[2], 16)
            .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?;
        let guid_part4 = vec![
            u8::from_str_radix(&guid_parts[3][0..2], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[3][2..4], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][0..2], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][2..4], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][4..6], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][6..8], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][8..10], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
            u8::from_str_radix(&guid_parts[4][10..12], 16)
                .map_err(|_e| input.error("guids parts shall only contain hexdigis"))?,
        ];
        Ok(RegisterStruct {
            provider_name,
            guid_part1,
            guid_part2,
            guid_part3,
            guid_part4,
        })
    }
}

struct WriteInput {
    event_name: Vec<u8>,
    fields: Vec<FieldInput>,
}

struct FieldInput {
    field_name: Vec<u8>,
    value: Expr,
    field_type: ExprPath,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let event_name: LitByteStr = input.parse()?;
        let event_name = event_name.value();
        input.parse::<Token![,]>()?;
        let fields: Punctuated<FieldInput, Token![,]> =
            input.parse_terminated(FieldInput::parse)?;
        let fields = fields.into_iter().collect();
        Ok(WriteInput { event_name, fields })
    }
}

impl Parse for FieldInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let field_name: LitByteStr = content.parse()?;
        let field_name = field_name.value();
        content.parse::<Token![,]>()?;
        let value: Expr = content.parse()?;
        content.parse::<Token![,]>()?;
        let field_type: ExprPath = content.parse()?;
        Ok(FieldInput {
            field_name,
            value,
            field_type,
        })
    }
}
