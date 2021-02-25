use yew_router::{switch::Permissive, Switch};

#[derive(Debug, Clone, PartialEq, Switch)]
pub enum AppRouter {
    #[to = "/index/i"]
    Index,
    #[to = "/index/r"]
    Indexr,
    #[to = "/"]
    Root,
    #[to = "/not_found"]
    PageNotFound(Permissive<String>),
}
