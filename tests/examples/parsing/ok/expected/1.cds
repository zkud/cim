entity Tests {
  key ID: String;
  ParentObjectID: String;
  ParentObjectIDLimited: String(10);
  Relationships: Association to ToRole_Relationship on ...;
  WorkingTimes: Association to ToRole_WorkingTime on ...;
}
entity RelationshipDetail {
  key ObjectID: String;
  RelationID: String;
  ParentObjectID: String;
  RelationType: String;
  PredecTaskID: String;
  SuccTaskID: String;
}
