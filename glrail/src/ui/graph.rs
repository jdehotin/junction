use imgui_sys_bindgen::sys::*;
use imgui_sys_bindgen::json::*;
use imgui_sys_bindgen::text::*;
use crate::app::*;
use crate::model::*;
use crate::view::*;
use crate::scenario::*;
use crate::infrastructure::*;
use crate::selection::*;
use crate::dgraph::*;
use crate::command_builder::*;
use std::ptr;
use std::ffi::CString;
use const_cstr::const_cstr;

use imgui_sys_bindgen::sys::ImVec2;

pub fn graph(size: ImVec2, app :&mut App) -> bool {
        let canvas_bg = 60 + (60<<8) + (60<<16) + (255<<24);
    let line_col  = 208 + (208<<8) + (175<<16) + (255<<24);
    let tvd_col  = 175 + (255<<8) + (175<<16) + (255<<24);
    let selected_col  = 175 + (175<<8) + (255<<16) + (255<<24);
    let line_hover_col  = 255 + (50<<8) + (50<<16) + (255<<24);
    // TODO make some colors config struct

    unsafe {

    let io = igGetIO();
    let mouse_pos = (*io).MousePos;

  igBeginChild(const_cstr!("Graph").as_ptr(), size, false, 0);
  let capture_canvas_key = igIsWindowFocused(0);

  let draw_list = igGetWindowDrawList();
  igText(const_cstr!("Here is the graph:").as_ptr());

  // we are in the graph mode, so we should have a selected dispatch
  let history = match app.model.view.selected_scenario {
      SelectedScenario::Dispatch(d) => {
          if let Some(Scenario::Dispatch(Dispatch { history: Derive::Ok(h), .. })) 
              = app.model.scenarios.get(d) { Some(h) } else { None }
      },
      SelectedScenario::Usage(u,Some(d)) => {
          if let Some(Scenario::Usage(_, Derive::Ok(dispatches))) 
              = app.model.scenarios.get(d) { 
                  if let Some(Dispatch { history: Derive::Ok(h), .. }) 
                      = dispatches.get(d) { Some(h) } else { None }
              } else { None }
      },
      _ => None,
  };


  igEndChild();

  capture_canvas_key
    }
}
