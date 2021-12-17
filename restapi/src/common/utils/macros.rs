// because actix_web is single-threaded, the async trait does not required to be Send.
#[macro_export]
macro_rules! define_steps {
    ($(async fn $name:ident ( $( $arg_name:ident : $arg_type:ty),* $(,)? ) -> $return:ty; )*) => {
        #[cfg(test)]
        #[async_trait(?Send)]
        pub trait MockSteps {
            $(
                async fn $name(&self, $($arg_name : $arg_type),* ) -> $return;
            )*
        }
        #[cfg(test)]
        #[derive(Clone, Copy)]
        pub struct Steps<'a>(&'a dyn MockSteps);
        #[cfg(test)]
        impl<'a> std::ops::Deref for Steps<'a> {
            type Target = dyn MockSteps + 'a;
            fn deref(&self) -> &Self::Target {
                self.0
            }
        }
        #[cfg(test)]
        impl<'a, M: MockSteps + 'a> std::convert::From<&'a M> for Steps<'a> {
            fn from(mock: &'a M) -> Self {
                Steps(mock)
            }
        }
        #[cfg(test)]
        impl<'a> Steps<'a> {
            pub fn from_ctx(_: &'a crate::common::utils::Context) -> Steps<'a> {
                panic!("`from_deps` is unavaliable in test env")
            }
        }

        #[cfg(not(test))]
        #[derive(Clone, Copy)]
        pub struct Steps<'a>(&'a crate::common::utils::Context);
        #[cfg(not(test))]
        impl<'a> Steps<'a> {
            pub fn from_ctx(ctx: &'a crate::common::utils::Context) -> Steps<'a> {
                Steps(ctx)
            }

            $(
                pub async fn $name(self, $($arg_name : $arg_type),* ) -> $return {
                    self::deps::$name(&self.0.deps, $($arg_name),*).await
                }
            )*
        }
    };
}
