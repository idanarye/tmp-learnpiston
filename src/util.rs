#[macro_export]
macro_rules! super_default {
    // Collect all attributes in parenthesis so we can parse them without interfering with $vis
    (
        @collect_attrs
        ( $( #[$attrs:meta] )* )
        #[$attr:meta]
        $( $tail:tt )*
    ) => { super_default! {
            @collect_attrs
            ( $( #[$attrs] )* #[$attr] )
            $( $tail )*
        }
    };

    // The actual rule that creates the struct
    (
        @collect_attrs
        ( $( #[$attr:meta] )* )
        $vis:vis struct $name:ident {
        $(
            $fvis:vis $field:ident : $typ:ty $( = $value:expr )*
        ),* $(,)*
    }) => {
        $( #[$attr] )*
        $vis struct $name {
            $(
                $fvis $field : $typ
             ),*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $field: super_default! { @decide_field_default $( $value )* }
                     ),*
                }
            }
        }
    };

    // Default value for fields that have their default value set as an expression
    (
        @decide_field_default $value:expr
    ) => {
        $value
    };

    // Default value for fields that don't have default expression - use the type's default
    (
        @decide_field_default
    ) => {
        Default::default()
    };

    // Entry point
    (
        $( $tt:tt )*
    ) => {
        super_default! { @collect_attrs () $( $tt )* }
    };
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
                        Input::Button(ButtonArgs { state: ButtonState::Press, button: Button::Keyboard(Key::$key), .. }) => {
                            self.$key_name = true;
                        }
                        Input::Button(ButtonArgs { state: ButtonState::Release, button: Button::Keyboard(Key::$key), .. }) => {
                            self.$key_name = false;
                        }
                     )*
                    _ => {}
                }
            }
        }
    }
}
