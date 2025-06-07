use proc_macro::{TokenStream};
use syn::{parse_macro_input, Data, DeriveInput, Ident};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn enum_to_diesel_integer(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let enum_names : Vec<Ident> = if let Data::Enum(data) = &input.data {
        data.variants.iter().map(|x| x.ident.clone()).collect()
    } else {
        panic!("Only enums are supported.");
    };


    let mut to_sql_tokens: proc_macro2::TokenStream = quote! {};
    let mut from_sql_tokens: proc_macro2::TokenStream = quote! {};

    enum_names.iter()
        .enumerate()
        .for_each(|(i, e)| {
            let i: i32 = i as i32;

            quote! {
                #name::#e=> diesel::serialize::ToSql::<diesel::sql_types::Integer, diesel::sqlite::Sqlite>::to_sql(&#i, out),
            }.to_tokens(&mut to_sql_tokens);

            quote! {
                #i => Ok(#name::#e),
            }.to_tokens(&mut from_sql_tokens);
        });
    

    return quote!{
        #[repr(i32)]
        #[derive(Debug, PartialEq, diesel::FromSqlRow, diesel::AsExpression, Eq, Clone)]
        #[diesel(sql_type = diesel::sql_types::Integer)]
        #input

        impl diesel::serialize::ToSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for #name {
            fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>) -> diesel::serialize::Result {
                match self {
                    #to_sql_tokens
                    
                }
            }
        }

        impl diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for #name {
            fn from_sql(bytes: diesel::sqlite::SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
                match <i32 as diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite>>::from_sql(bytes)? {
                    #from_sql_tokens
                    _ => Err("Unrecognized enum variant".into()),
                }
            }
        }
    }.into();
}