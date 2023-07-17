use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub count: u32,
    pub prev: Option<Url>,
    pub next: Option<Url>,
}

pub trait PagedResponse {
    fn previous(&self) -> Option<Url>;
    fn next(&self) -> Option<Url>;
}

pub(crate) mod macros {
    macro_rules! implement_paged_response  {
        ($($name:ident,$type:ident,$member:ident),+) => {$(
            impl PagedResponse for $name {
                fn previous(&self) -> Option<Url> {
                    self.pagination.prev.clone()
                }

                fn next(&self) -> Option<Url> {
                    self.pagination.next.clone()
                }
            }

         impl IntoIterator for $name {
             type Item = $type;
             type IntoIter = std::vec::IntoIter<Self::Item>;

             fn into_iter(self) -> Self::IntoIter {
                 self.$member.into_iter()
             }
         }

         impl<'iter> IntoIterator for &'iter $name {
             type Item = &'iter $type;
             type IntoIter = Iter<'iter, $type>;

             fn into_iter(self) -> Self::IntoIter {
                 self.$member.iter()
             }
         }

        )+};
        }

    pub(crate) use implement_paged_response;
}
