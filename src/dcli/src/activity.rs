use crate::mode::Mode;
use crate::manifest::activitydefinition::DestinyActivityDefinitionData;

#[derive(Debug, Clone)]
pub struct Activity {
    pub id:u32,
    pub name:String,

    //icon
    pub icon_path:String,

    //pgcrImage
    pub image_path:String,
    pub description:String,

    pub mode:Mode,
}

impl Activity {
    pub fn from_activity_definition_data(data:DestinyActivityDefinitionData) -> Activity {
        Activity {
            id:data.id,
            name:data.display_properties.name,
        
            //icon
            icon_path:data.display_properties.icon_path,
        
            //pgcrImage
            image_path:data.pgcr_image,
            description:data.display_properties.description,
            mode:data.direct_activity_mode_type,
        }
    }
}

