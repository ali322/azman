extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Dao)]
pub fn dao_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_dao(&ast)
}

fn impl_dao(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      #[async_trait]
      impl Dao for #name{
        async fn find_one(w: &Wrapper) -> Result<Self, DBError> {
          let w = w.clone().order_by(true, &["id"]).limit(1);
          POOL.fetch_by_wrapper::<Self>(&w).await
        }
        async fn find_list(w: &Wrapper) -> Result<Vec<Self>, DBError> {
            POOL.fetch_list_by_wrapper(w).await
        }
        async fn create_one(&self) -> Result<i64, DBError> {
            let created = POOL.save(&self, &[]).await?;
            Ok(created.last_insert_id.unwrap())
        }
        async fn update_one(&self, w: &Wrapper) -> Result<u64, DBError> {
            POOL.update_by_wrapper(&self, w, &[]).await
        }
        async fn delete_one(w: &Wrapper) -> Result<u64, DBError> {
            POOL.remove_by_wrapper::<Self>(w).await
        }
      }
    };
    gen.into()
}
