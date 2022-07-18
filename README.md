# **C**DS **I**mplementation generation by a **M**etadata document

This is a simle CLI program to generate KISS CAP CDS implementations from OData (both v2 and v4 are supported) Metadata documents.

```console
foo@bar:~$ cim metadata.xml
CIM 0.1.0
Reading metadata.xml...
entity ProjectElemDetail {
  ObjectID: String;
  ParentObjectID: String;
  HierarchyNodeLevel: Integer;
  DrillDownState: String;
  Magnitude: Integer;
  StartDate: DateTime;
  EndDate: DateTime;
}
entity RelationshipDetail {
  ObjectID: String;
  RelationID: String;
  ParentObjectID: String;
  RelationType: String;
  PredecTaskID: String;
  SuccTaskID: String;
}
...
```

## Installation

To use a tool compile the project on your local machine from sources and install with ```cargo install```, then use where it suits :).

## License

[MIT](LICENSE)
