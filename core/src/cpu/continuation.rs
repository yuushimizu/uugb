use super::CpuContext;

#[must_use]
pub enum Continuation<T> {
    Return(T),
    Continue(Box<dyn FnOnce(&mut dyn CpuContext) -> Continuation<T>>),
    Tick(Box<Continuation<T>>),
}

impl<T: 'static> Continuation<T> {
    pub fn just(value: T) -> Self {
        Self::Return(value)
    }

    pub fn new<F: FnOnce(&mut dyn CpuContext) -> Continuation<T> + 'static>(f: F) -> Self {
        Self::Continue(Box::new(f))
    }

    pub fn ticked(self) -> Self {
        Self::Tick(Box::new(self))
    }

    pub fn then<R: 'static, F: FnOnce(&mut dyn CpuContext, T) -> Continuation<R> + 'static>(
        self,
        f: F,
    ) -> Continuation<R> {
        use Continuation::*;
        Continuation::new(move |context| match self {
            Return(value) => f(context, value),
            Continue(next) => next(context).then(f),
            Tick(next) => next.then(f).ticked(),
        })
    }

    pub fn map<R: 'static, F: FnOnce(&mut dyn CpuContext, T) -> R + 'static>(
        self,
        f: F,
    ) -> Continuation<R> {
        self.then(|context, value| Continuation::just(f(context, value)))
    }

    pub fn tick(self) -> Self {
        self.then(|_context, value| Self::just(value).ticked())
    }
}
