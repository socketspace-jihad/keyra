@0xa438d2b112a6e42c;

struct MapEntry {
  key   @0 :Text;
  value @1 :Types;
}

struct Types {
  kind :union {
    stringVal @0 :Text;
    i64Val    @1 :Int64;
    i32Val    @2 :Int32;
    i16Val    @3 :Int16;
    i8Val     @4 :Int8;
    u64Val    @5 :UInt64;
    u32Val    @6 :UInt32;
    u16Val    @7 :UInt16;
    u8Val     @8 :UInt8;
    f64Val    @9 :Float64;
    f32Val    @10 :Float32;
    mapVal    @11 :List(MapEntry);
    arrayVal  @12 :List(Types);
  }
}

interface Data {
  receive @0 (key :Text, value :Types);
  list @1 ();
  get @2 (key :Text)->(value :Types);
}
