use super::*;
pub trait Dispatch<T> {
    fn send(&self, action: T);
}

pub trait RoutePage<T>
where
    T: Routable,
{
    fn goto(&self, route: T, id: Option<String>);
}

impl<T> RoutePage<T> for Navigator
where
    T: Routable,
{
    fn goto(&self, route: T, id: Option<String>) {
        self.push_with_query(&route, &UrlQuery { id })
            .expect("should be able to route page");
    }
}

impl<T, K> Dispatch<K> for UseReducerHandle<T>
where
    T: Reducible<Action = K>,
{
    fn send(&self, action: K) {
        self.dispatch(action);
    }
}
