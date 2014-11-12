/*!

Supervisor implementation

# Supervisor

Similar in functionality to an erlang supervisor, this contains both a generic
`Supervisor` type as well as a `Sup` trait for implementing your own
`Supervisor`s.

*/

use std::default::Default;

trait Sup {
    fn start(&self) -> Self;
}

#[deriving(Show, PartialEq, Eq)]
pub enum RestartStrategy {
    OneForOne,
    OneForAll,
    RestForOne,
}

#[deriving(Show, PartialEq, Eq)]
pub struct SupervisorConfig {
    restart_strategy: RestartStrategy,
    max_restart_freq: (uint, uint)
}

impl Default for SupervisorConfig {
    fn default() -> SupervisorConfig {
        SupervisorConfig {
            restart_strategy: OneForOne,
            max_restart_freq: (1u, 60u),
        }
    }
}

#[deriving(Show, PartialEq, Eq)]
pub struct Supervisor {
    config: SupervisorConfig
}

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor {
            config: Default::default()
        }
    }

    pub fn with_config(config: SupervisorConfig) -> Supervisor {
        Supervisor {
            config: config
        }
    }

    pub fn restart_strategy(&mut self, rs: RestartStrategy) -> Supervisor {
        self.config.restart_strategy = rs;
        *self
    }

    pub fn max_restart_freq(&mut self, mrf: (uint, uint)) -> Supervisor {
        self.config.max_restart_freq = mrf;
        *self
    }
}

impl Sup for Supervisor {
    fn start(&self) -> Supervisor {
        *self
    }
}

#[cfg(test)] mod test {
    use super::*;
    use std::default::Default;

    #[test]
    fn test_new() {
        assert_eq!(Supervisor::new(), Supervisor {
            config: SupervisorConfig {
                restart_strategy: OneForOne,
                max_restart_freq: (1u, 60u),
            }
        })
    }

    #[test]
    fn test_default_config() {
        assert_eq!(
                Supervisor::with_config(Default::default()),
                Supervisor {
                    config: SupervisorConfig {
                        restart_strategy: OneForOne,
                        max_restart_freq: (1u, 60u),
                    }
                }
        )
    }

    #[test]
    fn test_with_config() {
        assert_eq!(
                    Supervisor::with_config(SupervisorConfig {
                        restart_strategy: OneForAll,
                        max_restart_freq: (1u, 60u),
                    }),
                    Supervisor {
                        config: SupervisorConfig {
                            restart_strategy: OneForAll,
                            max_restart_freq: (1u, 60u),
                        }
                    }
        );
    }

    #[test]
    fn test_builder_restart_strategy() {
        assert_eq!(
                    Supervisor::new()
                        .restart_strategy(RestForOne)
                    ,
                    Supervisor {
                        config: SupervisorConfig {
                            restart_strategy: RestForOne,
                            max_restart_freq: (1u, 60u),
                        }
                    }
        );
    }

    #[test]
    fn test_builder_max_restart_freq() {
        assert_eq!(
                    Supervisor::new()
                        .max_restart_freq((2u, 120u))
                    ,
                    Supervisor {
                        config: SupervisorConfig {
                            restart_strategy: OneForOne,
                            max_restart_freq: (2u, 120u),
                        }
                    }
        );
    }
}

