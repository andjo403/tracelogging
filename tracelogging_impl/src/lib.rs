#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitByteStr, LitStr, Token};

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

/// Add one to an expression.
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
            use winapi::{
                shared::{evntprov, guiddef, winerror},
                um::{
                    cguid::GUID_NULL,
                    winnt::{PVOID, ULONGLONG},
                },
            };
            let mut handle: evntprov::REGHANDLE = 0;
            let guid = guiddef::GUID {
                Data1: #guid_part1,
                Data2: #guid_part2,
                Data3: #guid_part3,
                Data4: [#(#guid_part4),*],
            };

            let mut result =
                unsafe { evntprov::EventRegister(&guid, None, std::ptr::null_mut(), &mut handle) };

            if result == winerror::ERROR_SUCCESS {
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
                    result = evntprov::EventSetInformation(
                        handle,
                        evntprov::EventProviderSetTraits,
                        &event_info as *const _ as PVOID,
                        std::mem::size_of::<EventInformation>() as u32,
                    );
                }
                if result != winerror::ERROR_SUCCESS {
                    println!("EventSetInformation failed with '{}'", result);
                }
            } else {
                println!("register failed with '{}'", result);
            }
            unsafe {
                trace_logging::HANDLE = Some(handle);
            }
        }
    })
}
