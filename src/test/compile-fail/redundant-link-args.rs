// error-pattern:library 'm' already added: can't specify link_args.

/* I think it should undefined to have multiple modules that link in the same
  library, but provide different link arguments. Unfortunately we don't track
  link_args by module -- they are just appended as discovered into the crate
  store -- but for now, it should be an error to provide link_args on a module
  that's already been included (with or without link_args). */

#[link_name= "m"]
#[link_args="-foo"]             // this could have been elided.
extern mod m1 {
    #[legacy_exports];
}

#[link_name= "m"]
#[link_args="-bar"]             // this is the actual error trigger.
extern mod m2 {
    #[legacy_exports];
}
