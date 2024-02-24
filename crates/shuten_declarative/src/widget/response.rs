use crate::tree::WidgetId;

pub type NoResponse = ();

#[derive(Debug)]
pub struct Response<T = NoResponse> {
    inner: T,
    id: WidgetId,
}

impl<T> Response<T> {
    pub(crate) const fn new(id: WidgetId, inner: T) -> Self {
        Self { inner, id }
    }

    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> Response<U> {
        Response {
            inner: map(self.inner),
            id: self.id,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub const fn id(&self) -> WidgetId {
        self.id
    }
}

impl<T> std::ops::Deref for Response<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for Response<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
