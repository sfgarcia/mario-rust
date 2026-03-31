/// Estado del teclado compartido entre el game loop y los event listeners.
#[derive(Default, Clone)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub restart: bool,
    /// Para detectar flanco de subida del salto (evitar salto continuo).
    pub jump_pressed: bool,
    /// Flanco de subida del restart (evitar restart continuo al mantener R).
    pub restart_pressed: bool,
}
