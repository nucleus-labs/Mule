
# Mule

Mule is the software component of an in-progress rewrite of the [AgOpenGPS](https://github.com/farmerbriantee/AgOpenGPS)
project that aims to maximize the project's accessibility, including the UI, upgrading hardware
requirements from "Windows Laptop / Tablet" to
"Any mobile device running Windows / Linux / Android / iOS", etc.

# Plans

## Progress Towards Prototype

- [ ] Identify components of the original AOG software to rewrite
    - Active
    - AgIO
    - GPS - Main Screen ([`FormGPS`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Forms/FormGPS.cs))
        - Renderer Camera ([`CCamera`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CCamera.cs))
        - Renderer World Grid ([`CWorldGrid`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CWorldGrid.cs))
        - Renderer Triangle Strip<sup>[[1]](#)</sup> ([`List<CPatches>`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CPatches.cs))
        - Renderer Font ([`CFont`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CFont.cs))
        - Renderer Brightness Controller ([`CWindowsSettingsBrightnessController`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CBrightness.cs))
        - Audio Player ([`CSound`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CSound.cs))
        - Resource Manager ([`ResourceManager`](https://learn.microsoft.com/en-us/dotnet/api/system.resources.resourcemanager?view=net-8.0))
        - Simulator NMEA ([`CSim`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CSim.cs))
        - Communication Modules Container ([`CModuleComm`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CModuleComm.cs))
        - Communication NMEA Parser ([`CNMEA`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CNMEA.cs))
        - Vehicle Specs ([`CVehicle`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CVehicle.cs))
        - Vehicle Tool ([`CTool`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CTool.cs))
        - Navigation ([`CGuidance`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CGuidance.cs))
        - Navigation Line ([`CABLine`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CABLine.cs))
        - Navigation Curve ([`CABCurve`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CABCurve.cs))
        - Navigation Headland Line ([`CHeadLine`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CHeadLine.cs))
        - Navigation Attitude and Heading Reference System ([`CAHRS`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CAHRS.cs))
        - Navigation Path ([`CRecordedPath`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CRecordedPath.cs))
        - Field Section<sup>[[1]](#)</sup> ([`CSection`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CSection.cs))
        - Field Context<sup>[[2]](#)</sup> ([`CFieldData`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CFieldData.cs))
        - Field Boundary ([`CBoundary`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CBoundary.cs))
        - Field Tramline Context ([`CTram`](https://github.com/farmerbriantee/AgOpenGPS/blob/209ccead1b9c9e468c13c441a4ef9fbed5e5c49f/SourceCode/GPS/Classes/CTram.cs))
            
- [ ] Manually program a UI using a provided mock-up as reference

<sup>[[1]](#)</sup>
These are named singularly despite being plurals
<br/>
<sup>[[2]](#)</sup>
This contains a bunch of data related to the processing of a field, rather than static info about the field itself

## Progress towards Front-End Overhaul

- [X] Get Pinion into a working and usable state
- [ ] Get Crest into a working and usable state
    - [X] Finish writing the grammar for the parser
    - [ ] Develop an API for the parser that is easier to use than the one that's generated by
    the `pest` crate.
    - [ ] Implement standard CSS properties
    - [ ] Implement pseudo-classes
    - [ ] Implement at-rules
- [ ] Get Peacock into a working and usable state (glue)
    - EARLY (BODGE)
        - `build.rs` stage that generates .rs files for iced from XML files (templating can't be used?)
    - MATURE
        - [ ] Design a standardized Widget trait for traits that implement it to be converted into
        an `iced` trait
        - [ ] Implement iced widget structs for standard HTML elements (Container, Row, Column, etc)
        - [ ] add a 3d-canvas widget
- [ ] Put together a mock UI design for Mule's interface
- [ ] Use Peacock to create the UI design for Mule (no 3d-renderer)
- [ ] Implement the functionality behind the UI in rust

## Milestones

- [ ] Software prototype
- [ ] Front-end overhaul
- [ ] Hardware prototype
- [ ] Language files (for easy translations)

## Noteworthy

- [AgOpenGPS Simulation Proof-Of-Concept](https://github.com/GNSS-Stylist/AgOpenGPSSimPoC)
    - A simulation that uses the Godot game engine to model tractors' behavior in a virtual setting
