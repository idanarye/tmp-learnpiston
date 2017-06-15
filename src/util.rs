#[macro_export]
macro_rules! initialized_object {
    ( $name:ident 
      $( let $field:ident : $typ:ty = $value:expr ; )*
      ) => {
        pub struct $name {
            $(
                $field: $typ,
             )*
        }
        impl $name {
            pub fn new() -> $name {
                return $name {
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
