#[macro_export]
macro_rules! define_http_routes {
    (
        $(
            group (
                $(auth $auth_ty:ty;)?
                error $unexpected:ty;

                $(
                    $method:ident $path:literal $req:ty => $res:ty | $err:ty;
                )*
            );
        )*
    ) => {
        $(
            $crate::define_http_routes!(@impl_group $(auth $auth_ty;)? $unexpected; $(($method, $path, $req, $res, $err))*);
        )*
    };

    (@impl_group auth $auth_ty:ty; $unexpected:ty; $(($method:ident, $path:literal, $req:ty, $res:ty, $err:ty))*) => {
        $(
            impl $crate::http::client::HttpRequest<$res, $err, $unexpected> for $req {
                const ENDPOINT: &'static str = concat!("/", $path);
                const METHOD: $crate::http::client::RequestMethod = $crate::http::client::RequestMethod::$method;
            }

            impl Into<$crate::http::client::Response<$res, $err, $unexpected>> for $res
            {
                fn into(
                    self,
                ) -> $crate::http::client::Response<$res, $err, $unexpected>
                {
                    $crate::http::client::Response::Ok(self)
                }
            }

            impl Into<$crate::http::client::Response<$res, $err,$unexpected>> for $err
            {
                fn into(
                    self,
                ) -> $crate::http::client::Response<$res, $err,$unexpected>
                {
                    $crate::http::client::Response::Error(self)
                }
            }

        impl Into<$crate::http::client::Response<$res, $err, $unexpected>> for $unexpected
        {
            fn into(
                self,
            ) -> $crate::http::client::Response<$res, $err, $unexpected>
            {
                $crate::http::client::Response::UnexpectedError(self)
            }
        }


        impl $crate::http::client::AuthenticatedHttpRequest<$auth_ty> for $req {}
        )*
    };

    (@impl_group $unexpected:ty; $(($method:ident, $path:literal, $req:ty, $res:ty, $err:ty))*) => {
        $(
            impl $crate::http::client::HttpRequest<$res, $err, $unexpected> for $req {
                const ENDPOINT: &'static str = concat!("/", $path);
                const METHOD: $crate::http::client::RequestMethod = $crate::http::client::RequestMethod::$method;
            }

            impl Into<$crate::http::client::Response<$res, $err, $unexpected>> for $res
            {
                fn into(
                    self,
                ) -> $crate::http::client::Response<$res, $err, $unexpected>
                {
                    $crate::http::client::Response::Ok(self)
                }
            }

            impl Into<$crate::http::client::Response<$res, $err,$unexpected>> for $err
            {
                fn into(
                    self,
                ) -> $crate::http::client::Response<$res, $err,$unexpected>
                {
                    $crate::http::client::Response::Error(self)
                }
            }

        impl Into<$crate::http::client::Response<$res, $err, $unexpected>> for $unexpected
        {
            fn into(
                self,
            ) -> $crate::http::client::Response<$res, $err, $unexpected>
            {
                $crate::http::client::Response::UnexpectedError(self)
            }
        }
        )*
    };
}
