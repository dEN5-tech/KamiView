use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, parse_quote,
    ItemFn, FnArg, PatType, Type, punctuated::Punctuated, token::Comma
};

/// Атрибут-макрос для методов обновления компонента
/// 
/// Этот макрос генерирует:
/// - Enum для сообщений компонента
/// - Реализацию обработчика обновлений
/// - Конвертацию в основной тип Message
#[proc_macro_attribute]
pub fn component_update(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // Извлекаем имя структуры из имени функции
    // например, handle_image_loaded -> ImageLoaded
    let fn_name = &input.sig.ident;
    let struct_name = format_ident!("{}", extract_struct_name(fn_name));
    
    let vis = &input.vis;
    let block = &input.block;
    let generics = &input.sig.generics;
    
    // Извлекаем тип сообщения из аргументов функции
    let msg_type = extract_message_type(&input.sig.inputs)
        .unwrap_or_else(|| parse_quote!(()));
    
    // Генерируем реализацию
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    let expanded = quote! {
        // Оригинальная функция
        #vis #input

        // Добавляем метод в реализацию структуры
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn handle_message(&mut self, msg: &#msg_type) -> Command<Message> {
                #block
            }
        }
    };
    
    expanded.into()
}

// Вспомогательные функции
fn extract_message_type(inputs: &Punctuated<FnArg, Comma>) -> Option<Type> {
    inputs.iter().find_map(|arg| {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            Some((**ty).clone())
        } else {
            None
        }
    })
}

// Извлекает имя структуры из имени функции
// например: handle_image_loaded -> ImageLoaded
fn extract_struct_name(ident: &syn::Ident) -> String {
    let name = ident.to_string();
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() >= 2 {
        // Пропускаем "handle" и преобразуем остальные части
        parts[1..].iter()
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                    None => String::new(),
                }
            })
            .collect()
    } else {
        name
    }
}

/// Вспомогательный макрос для создания структур компонентов
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let component = parse_macro_input!(input as syn::DeriveInput);
    let name = &component.ident;
    
    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            state: ComponentState,
        }

        impl Component for #name {
            type Message = Message;

            fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
                match message {
                    Message::#name(msg) => self.handle_message(&msg),
                    _ => Command::none(),
                }
            }
        }
    };

    expanded.into()
}
