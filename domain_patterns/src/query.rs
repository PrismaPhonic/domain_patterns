/// Query is a simple marker trait that should be placed on query types which we plan to handle with
/// a QueryHandler (a struct that implements HandlesQuery)
pub trait Query {}

/// HandlesQuery is a trait that you apply to a struct, which knows how to handle a query.  A query
/// is a struct or enum that implements the Query trait, and is a parameter object that we can use
/// to construct a custom query.
pub trait HandlesQuery<T: Query> {
    type Result;

    fn handle(&mut self, query: T) -> Self::Result;
}
