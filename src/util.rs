#[macro_export]
macro_rules! initialized_object {
    (
        $vis:vis struct $name:ident {
        $(
            $field_vis:vis $field:ident : $typ:ty = $value:expr,
        )*
    }) => {
        // #[derive(Debug)]
        $vis struct $name {
            $(
                $field_vis $field : $typ
             ),*
        }

        impl $name {
            $vis fn new() -> Self {
                Self {
                    $(
                        $field: $value
                     ),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! key_map {
    ( $name:ident
      $( $key_name:ident = $key:ident ; )*
      ) => {
        struct $name {
            $(
                $key_name : bool
            ),*
        }

        impl $name {
            fn new() -> $name {
                return $name {
                    $(
                        $key_name: false
                     ),*
                }
            }

            fn update(&mut self, inp: &Input) {
                match *inp {
                    $(
                        Input::Press(Button::Keyboard(Key::$key)) => {
                            self.$key_name = true;
                        }
                        Input::Release(Button::Keyboard(Key::$key)) => {
                            self.$key_name = false;
                        }
                     )*
                    _ => {}
                }
            }
        }
    }
}
