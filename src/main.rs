use blue_engine::{header::{Engine, /*ObjectSettings,*/ WindowDescriptor, PowerPreference}, primitive_shapes::{triangle, self}};
use blue_engine_egui::{self, egui};


use std::process::exit;


pub mod object_settings;
pub mod sql;
mod practice;

struct EditorSettings
{
    width               : f32,
    height              : f32,
    range               : f32,
    slider_speed        : f32,
}
impl EditorSettings
{
    fn init() -> Self
    {
        Self
        {
            width               : 250f32,
            height              : 900f32,
            range               : 900_000_000f32,
            slider_speed        : 1f32,
        }
    }
}

pub struct Objects
{
    id          : u16,
    visible     : bool,
    selected    : bool,
    label       : (String, issues::Issues),
}
impl Objects
{
    fn init(id: u16) -> Self
    {
        Self
        {
            id,
            visible     : true,
            selected    : true,
            label       : (format!("Object {id}"), issues::Issues::init())
        }
    }
    fn change_choice(list: &mut [(Self, ObjectSettings)], choice_true: u16)
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            if i as u16 == choice_true
            {
                item.0.selected = true;
            }
            else
            {
                item.0.selected = false;
            }
        }
    }
    // When user deletes the objects, we need to re calculate ids
    fn recalculate_id(list: &mut  [(Self, ObjectSettings)])
    {
        for (i, item) in list.iter_mut().enumerate()
        {
            item.0.id = i as u16;
        }
    }
    // Checks for warnings and errors for labels and assigns the Issues variables appropriately

}

pub struct ObjectSettings
{
    object_type         : Vec<object_settings::radio_options::Fields>,
    position            : [object_settings::three_d_lables::Fields; 3],
    scale               : [object_settings::three_d_lables::Fields; 3],
    texture             : object_settings::texture::Fields,
}
impl ObjectSettings
{
    fn init() -> Self
    {
        Self
        {
            object_type         : object_settings::radio_options::init(&["Square", "Triangle", "Line"]),
            position            : object_settings::three_d_lables::Fields::init(0f32),
            scale               : object_settings::three_d_lables::Fields::init(1f32),
            texture             : object_settings::texture::Fields::init(),
        }
    }
}

struct AlertWindow
{
    label       : &'static str,
    state       : bool,
}
impl AlertWindow
{
    fn init() -> [Self; 6]
    {
        [
            Self{label: "Open", state: false},
            Self{label: "New", state: false},
            Self{label: "💾 Save", state: false},
            Self{label: "Export settings", state: false},
            Self{label: "Import settings", state: false},
            Self{label: "⚙ Settings", state: false},
        ]
    }

    fn whats_enabled(alert_window: &[Self]) -> &'static str
    {
        for list in alert_window.iter()
        {
            if list.state == true
            {
                return list.label;
            }
        }

        return "";
    }
}


mod issues
{
    pub struct Issues
    {
        pub warning         : bool,
        pub error           : bool,
    }
    impl Issues
    {
        pub fn init() -> Self
        {
            Self
            {
                warning     : false,
                error       : false,
            }
        }
    }

    pub fn output_symbols() -> (&'static str, &'static str)
    {
        ("⚠", "⛔")
    }

    pub mod issue_checks
    {
        pub fn labels(objects: &mut [(crate::Objects, crate::ObjectSettings)])
        {
            if objects.len() == 1
            {
                objects[0].0.label.1.error = false;
                return;
            }
            for i in 0..objects.len()
            {
                for j in 0..objects.len()
                {
                    if i != j
                    {
                        if objects[i].0.label.1.error != true && objects[i].0.label.0 == objects[j].0.label.0
                        {
                            objects[i].0.label.1.error = true;
                            break;
                        }
                        else
                        {
                            objects[i].0.label.1.error = false;    
                        }
                    }
                }
            }
        }
    }
}

struct Debug
{
    practice            : bool,
    resolution          : bool,
}

struct Sql
{
    objects             : sql::objects::Sql,
    //projects            : sql::projects::Sql,
}
impl Sql
{
    fn init() -> Self
    {
        Self
        {
            objects             : sql::objects::Sql::init(),
            //projects            : sql::projects::Sql::init(),
        }
    }
}

fn main()
{
    let editor_settings = EditorSettings::init();

    let debug = Debug
    {
        practice        : false,
        resolution      : true,
    };

    if debug.practice == true
    {
        println!("\n--------------Practice Mode!!!!!--------------\n");
        practice::main();
        println!("\n--------------End of practice!!!!!--------------\n");
        exit(0);
    }
    
    let mut alert_window = (false, AlertWindow::init());

    let mut engine = Engine::new(
        WindowDescriptor
        {
            width               : if debug.resolution == true {1280} else {1920},
            height              : if debug.resolution == true {720} else {1080},
            title               : "Blue Flame",
            decorations         : true,
            resizable           : true,
            power_preference    : PowerPreference::LowPower
        }).unwrap();


    triangle("triangle", blue_engine::header::ObjectSettings::default(), &mut engine).unwrap();
    //primitive_shapes::square("square", blue_engine::header::ObjectSettings::default(), &mut engine).unwrap();


    // Start the egui context
    //let gui_context = blue_engine::header::egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    let mut color = [1f32, 1f32, 1f32, 1f32];



    // sql Variables
    let mut sql = Sql::init();

    // objects
    //let mut objects = vec![(Objects::init(0), ObjectSettings::init())];
    let mut objects: Vec<(Objects, ObjectSettings)> = Vec::new();

    struct MyGUI
    {
        // Some variables you'd desire
        color: [f32; 4],
    }

    
    sql.objects.load(&mut objects);

    let my_gui = MyGUI
    {
        color: [1f32, 1f32, 1f32, 1f32],
    };
    // Start the egui context
    let gui_context = blue_engine_egui::EGUI::new(&engine.event_loop, &mut engine.renderer);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    engine.update_loop(move |_, window, gameengine_objects, _, _, plugins|
    {
        // Label error checking
        issues::issue_checks::labels(&mut objects);

        // obtain the plugin
        let egui_plugin = plugins[0]
        // downcast it to obtain the plugin
        .downcast_mut::<blue_engine_egui::EGUI>()
        .expect("Plugin not found");

        // ui function will provide the context
        egui_plugin.ui(|ctx|
        {
            // One of the settings menu
            egui::Window::new(AlertWindow::whats_enabled(&alert_window.1))
            .fixed_pos(egui::pos2(400f32, 50f32))
            .fixed_size(egui::vec2(100f32, 200f32))
            .open(&mut alert_window.0)
            .show(ctx, |ui| ui.label(""));
            {
                
            }
            // Menu bar
            egui::TopBottomPanel::top("Menu Bar").show(ctx, |ui|
            {
                ui.set_enabled(!alert_window.0);

                egui::menu::bar(ui, |ui|
                {
                    ui.menu_button("Menu", |ui|
                    {
                        for list in alert_window.1.iter_mut()
                        {
                            // Individual elements after clicking on "Menu"
                            if ui.button(list.label).clicked()
                            {
                                if list.label == "💾 Save"
                                {
                                    sql.objects.save(&objects);
                                    break;
                                }

                                alert_window.0 = true;
                                list.state = true;
                            }
                            else if alert_window.0 == false
                            {
                                list.state = false;
                            }
                        }

                    });
                    ui.menu_button("About", |ui|
                    {
                        //if ui.bu
                    });

                });
            });

            // Left panel
            egui::SidePanel::left("Objects").show(ctx, |ui|
            {
                ui.set_enabled(!alert_window.0);

                ui.set_width(ui.available_width());

                ui.horizontal(|ui|
                {
                    // Create new object
                    if ui.button("➕ Create Object").clicked()
                    {
                        let len = objects.len() as u16;

                        objects.push((Objects::init(len), ObjectSettings::init()));
                        Objects::change_choice(&mut objects, len);
                    }
                    if ui.button("💾 Save").clicked()
                    {
                        sql.objects.save(&objects);
                    }
                });


                ui.separator();

                // Displays all objects button
                for i in 0..objects.len()
                {
                    ui.horizontal(|ui|
                    {
                        ui.collapsing(format!("id: {}", &objects[i].0.id), |ui|
                        {
                            ui.label("some stuff");
                        });
                        if ui.selectable_label(objects[i].0.selected, &objects[i].0.label.0).clicked()
                        {
                            Objects::change_choice(&mut objects, i as u16);
                        }
                        ui.checkbox(&mut objects[i].0.visible, "");
                        if objects[i].0.visible == true
                        {
                            ui.label("👁");
                        }

                        // Checks if variable names are correct or not
                        // Warnings
                        if objects[i].0.label.1.warning == true
                        {
                            ui.label(issues::output_symbols().0);
                        }
                        // Errors
                        if objects[i].0.label.1.error == true
                        {
                            ui.label(issues::output_symbols().1);
                        }
                    });
                }
            });

            // Right side
            egui::SidePanel::right("Object Settings").show(ctx, |ui|
            {
                ui.set_enabled(!alert_window.0);

                ui.set_width(ui.available_width());


                for object in objects.iter_mut()
                {

                    if object.0.selected == true
                    {
                        ui.label(format!("Object name: {} {}",
                            if object.0.label.1.warning == true {issues::output_symbols().0} else {""},
                            if object.0.label.1.error == true {issues::output_symbols().1} else {""},
                        ));
                        ui.add(egui::TextEdit::singleline(&mut object.0.label.0));
                    }
                }

                for object in objects.iter_mut()
                {
                    if object.0.selected == true
                    {
                        ui.label("Object type");
                        ui.horizontal(|ui|
                        {
                            for i in 0..object.1.object_type.len()
                            {
                                if ui.radio(object.1.object_type[i].status, object.1.object_type[i].name).clicked()
                                {
                                    object_settings::radio_options::change_choice(&mut object.1.object_type, i as u8);
                                }
                            }
                        });
                        ui.separator();

                        ui.label("TextureMode");
                        ui.label("Location of Texture");
                        ui.add(egui::TextEdit::singleline(&mut object.1.texture.data));


                        // Radio buttons for texturemodes
                        for i in 0..object.1.texture.mode.len()
                        {
                            if ui.radio(object.1.texture.mode[i].status, object.1.texture.mode[i].name).clicked()
                            {
                                object_settings::radio_options::change_choice(&mut object.1.texture.mode, i as u8);
                            }
                        }
                        ui.separator();

                        ui.label("Position");
                        ui.horizontal(|ui|
                        {
                            for position in object.1.position.iter_mut()
                            {
                                ui.label(format!("{}:", position.axis as char));
                                ui.add(egui::DragValue::new(&mut position.value).speed(editor_settings.slider_speed));
                            }
                        });
                        ui.separator();

                        ui.label("Scale");
                        ui.horizontal(|ui|
                        {
                            for scale in object.1.scale.iter_mut()
                            {
                                ui.label(format!("{}:", scale.axis as char));
                                ui.add(egui::DragValue::new(&mut scale.value).speed(editor_settings.slider_speed));
                            }
                        });


                    }
                }

                for _ in 0..2
                {
                    ui.separator();
                }

                ui.horizontal(|ui|
                {
                    for i in 0..objects.len()
                    {
                        if objects[i].0.selected == true
                        {
                            if ui.button("🗑 Delete").clicked()
                            {
                                objects.remove(i);
                                Objects::recalculate_id(&mut objects);
                                break;
                            }
                        }

                    }

                });

                //println!("{:?}", response);
            });
            
        },
        &window);
            
        // Display shapes
        


        /*
        gameengine_objects[square]
        .change_color(color[0], color[1], color[2], color[3])
        .unwrap();
        */
        
    }).unwrap();


    
}