use ash::vk;
use std::ffi::CString;
use std::ptr;

use glfw::{Action, Context, Glfw, Key};
use ash;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;


pub struct VulkanApp {
    instance : ash::Instance,
    entry : ash::Entry
}

impl VulkanApp {
    pub fn new(glfw : &mut glfw::Glfw) -> VulkanApp {
        unsafe {
            let entry = ash::Entry::new().unwrap();
            let instance = VulkanApp::create_instance(&entry, glfw);
            Self {
                entry,
                instance
            }
        }
    }

    fn create_instance(
        entry : &ash::Entry,
        glfw : &mut glfw::Glfw
    ) -> ash::Instance {
        let app_name = CString::new("Hello app").unwrap();
        let engine_name = CString::new("Rewin engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type : vk::StructureType::APPLICATION_INFO,
            p_next : ptr::null(),
            p_application_name : app_name.as_ptr(),
            p_engine_name : engine_name.as_ptr(),
            application_version : vk::make_api_version(1, 0, 0, 0),
            engine_version : vk::make_api_version(1, 0, 0, 0),
            api_version : vk::make_api_version(1, 0, 92, 0)
        };

        let extension_names = required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

#[cfg(all(windows))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}
