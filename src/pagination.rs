use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub count: u32,
    pub prev: Option<Url>,
    pub next: Option<Url>,
}

pub trait PagedResponse<T> {
    fn get_items(&self) -> &Vec<T>;
    fn get_pagination(&self) -> &Pagination;

    fn previous(&self) -> Option<Url> {
        self.get_pagination().prev.clone()
    }

    fn next(&self) -> Option<Url> {
        self.get_pagination().next.clone()
    }
}

pub(crate) mod macros {
    macro_rules! paged_iterator {
        ($($name:ident,$type:ident),+) => {$(

         impl<'iter> IntoIterator for &'iter $name {
             type Item = &'iter $type;
             type IntoIter = std::slice::Iter<'iter, $type>;

             fn into_iter(self) -> Self::IntoIter {
                 self.get_items().iter()
             }
         }
        )+};
    }
    pub(crate) use paged_iterator;
}
