use serde::Deserialize;
use std::collections::HashMap;
use std::{fs};

#[derive(Debug, Deserialize)]
struct XMLDocument {
    pub Resources: XMLResources,
    pub Storages: XMLStorages,
    pub WorkOrders: XMLWorkOrders,
}

#[derive(Debug, Deserialize)]
struct XMLResources {
    pub Machines: XMLMachines,
    pub MachineGroups: XMLMachineGroups
}

#[derive(Debug, Deserialize)]
struct XMLStorages {
    pub Unit: XMLUnit,
}

#[derive(Debug, Deserialize)]
struct XMLUnit {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@Capacity")]
    pub Capacity: String,
    #[serde(rename = "@Type")]
    pub Type: String,
}

#[derive(Debug, Deserialize)]
struct XMLMachines {
    pub Station: Vec<XMLMachine>
}

#[derive(Debug, Deserialize)]
struct XMLMachineGroups {
    pub Station: Vec<XMLMachineGroup>
}

#[derive(Debug, Deserialize)]
struct XMLMachine {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@Capacity")]
    pub Capacity: String,
    
}

#[derive(Debug, Deserialize)]
struct XMLMachineGroup {
    #[serde(rename = "@id")]
    pub id: String,
    pub Machine: Vec<XMLMachineRef>

}

#[derive(Debug, Deserialize)]
struct XMLMachineRef {
    #[serde(rename = "@ref")]
    pub _ref: String

}

#[derive(Debug, Deserialize)]
struct XMLWorkOrders {
    pub WorkOrder: Vec<XMLWorkOrder>,
}

#[derive(Debug, Deserialize)]
struct XMLWorkOrder {
    #[serde(rename = "@Order")]
    pub id: String,
    pub Durations: XMLDurations,
    pub RequiredResources: XMLRequiredResources,
}

#[derive(Debug, Deserialize)]
struct XMLDurations {
    #[serde(rename = "@Start")]
    pub Start: String,
    #[serde(rename = "@End")]
    pub End: String,
}

#[derive(Debug, Deserialize)]
struct XMLRequiredResources {
    #[serde(rename = "@id")]
    pub id: String
}


fn parse_storages(doc: &XMLDocument) -> Vec<Storages> {
    doc.Storages.Unit.iter()
    .map(|storage| {
        Storages::with_capacity(storage.id.clone(), str::parse::<usize>(&storage.capacity).unwrap())
    }).collect()
}

fn parse_resources(doc: &XMLDocument) -> Vec<Resources> {
    doc.Resources.Machines.Machine.iter()
    .map(|machine| {
        Resources::with_capacity(machine.id.clone(), str::parse::<usize>(&machine.capacity).unwrap())
    }).collect()
}
