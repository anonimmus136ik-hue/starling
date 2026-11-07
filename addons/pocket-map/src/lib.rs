use horizon_mod_sdk::{Mod, Registration, render, storage, ui};
const SIZES:[f32;3]=[64.,88.,112.];
const SCALE:[f32;4]=[0.5,1.,2.,4.];
const COUNTS:[u8;6]=[2,3,4,4,2,2];
struct Map { choices:[u8;6], open:bool }
impl Map {
 fn apply(&self){
 let [visible,size,scale,corner,rotate,round]=self.choices;
 let corner=[render::Corner::TopLeft,render::Corner::TopRight,render::Corner::BottomLeft,render::Corner::BottomRight][corner as usize];
 render::set_minimap((visible==1).then_some(render::Minimap{corner,size:SIZES[size as usize],blocks_per_pixel:SCALE[scale as usize],rotate:rotate==1,round:round==1}));
 }
 fn draw(&self){
 let [v,s,z,c,r,f]=self.choices;
 let labels=[format!("Карта: {}",if v==1{"вкл"}else{"выкл"}),format!("Размер: {} px",SIZES[s as usize]),format!("Масштаб: {} бл./пикс.",SCALE[z as usize]),format!("Угол: {}",["верхний левый","верхний правый","нижний левый","нижний правый"][c as usize]),format!("Ориентация: {}",if r==1{"по игроку"}else{"север сверху"}),format!("Форма: {}",if f==1{"круг"}else{"квадрат"})];
 let widgets=labels.into_iter().enumerate().map(|(i,label)|ui::Widget::Button(ui::ButtonWidget{id:i as u32+1,label})).collect::<Vec<_>>();
 ui::set_settings(&widgets);
 if self.open{ui::open_screen(&ui::Screen{title:"Pocket Map".into(),widgets});}
 }
}
impl Mod for Map {
 fn init()->Result<(Self,Registration),String>{
 let mut choices=[1,1,1,1,0,0];
 if let Some(data)=storage::get("settings-v1"){if data.len()==6 && data.iter().zip(COUNTS).all(|(v,n)|*v<n){choices.copy_from_slice(&data);}}
 let m=Self{choices,open:false};m.apply();m.draw();Ok((m,Registration::new().command("pocketmap")))
 }
 fn command(&mut self,_:&str,_:&str)->Result<Option<String>,String>{self.open=true;self.draw();Ok(Some("Pocket Map".into()))}
 fn ui_select(&mut self,id:u32,row:u32){if row!=0 || !(1..=6).contains(&id){return;}let i=id as usize-1;self.choices[i]=(self.choices[i]+1)%COUNTS[i];let _=storage::set("settings-v1",&self.choices);self.apply();self.draw();}
 fn ui_settings_opened(&mut self){self.open=false;self.draw();}
 fn ui_closed(&mut self){self.open=false;}
}
horizon_mod_sdk::export_mod!(Map);
