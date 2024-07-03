pub mod player;

use godot::engine::Engine;
use godot::prelude::*;
use godot::classes::{Button, IButton, PackedScene};
use godot::obj::Gd;

#[derive(GodotClass)]
#[class(init, base=Object)]
struct GlobalDataSingleton {
    deck: Option<Vec<player::Carta>>,

    base: Base<Object>,
}

#[godot_api]
impl GlobalDataSingleton {
    #[func]
    fn init_deck(&mut self) {
        unimplemented!()
    }

    #[func]
    fn deal_cards(&mut self) -> [player::Carta; 3] {
        unimplemented!()
    }
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                StringName::from("GlobalDataSingleton"),
                GlobalDataSingleton::new_alloc().upcast(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = StringName::from("GlobalDataSingleton");

            let singleton = engine
                .get_singleton(singleton_name.clone())
                .expect("cannot retrieve GlobalDataSingleton");

            engine.unregister_singleton(singleton_name);
            singleton.free();
        }
    }
}

#[derive(GodotClass)]
#[class(base=Button)]
struct MainMenuButton {
    base: Base<Button>,
    #[export]
    scene_to_load: Option<Gd<PackedScene>>,
}

#[godot_api]
impl IButton for MainMenuButton {
    fn init(base: Base<Button>) -> Self {
        Self { base, scene_to_load: None }
    }

    fn pressed(&mut self) {
        let Some(scene_to_load) = self.get_scene_to_load() else {
            return
        };

        self.base_mut()
            .get_tree().expect("no scenetree")
            .change_scene_to_packed(scene_to_load);
    }
}

#[godot_api]
impl MainMenuButton {
    #[signal]
    fn load_scene(scene: GString);
}

