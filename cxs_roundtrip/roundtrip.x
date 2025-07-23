namespace test {

struct Foo {
    unsigned int a;
};


// test cases from xdrtest.x
enum senum {
  SE_NEGATIVE = -1,
  SE_POSITIVE = 1
};

union sunion switch (senum d) {
  case SE_NEGATIVE:
    bool neg;
  case SE_POSITIVE:
    int pos;
  default:
    void;
};

union sunion_no_void switch (senum d) {
  case SE_NEGATIVE:
    bool neg;
  case SE_POSITIVE:
    int pos;
};

union uunion switch (unsigned int d) {
  case 1:
    bool one;
  case 2:
    int two;
  // double not supported in wasm
  case 4:
    string four<>;
};

union uunion_bool switch (bool d) {
  case TRUE:
    bool one;
  case FALSE:
    bool two;
};

struct test_recursive {
  string elem<>;
  test_recursive *next; // #[rust(box_ptr)]
  test_recursive nextvec<>;
};

const kBytestringMaxLen = 1024; // #[rust(const=u32)]

typedef opaque Bytestring<kBytestringMaxLen>;

typedef opaque SHA256Hash[32];

struct NonnegativeInt64SetAddArgs {
  hyper set_value; // int64_t
  hyper add_value; // int64_t
};

enum StorageDeltaType {
  kDeleteLast = 0,
  kBytestringSet = 1,
  kNonnegativeInt64SetAdd = 2
};

union StorageDelta switch (StorageDeltaType delta_type)
{
case kDeleteLast:
  void;
case kBytestringSet:
  Bytestring bstring;
case kNonnegativeInt64SetAdd:
  NonnegativeInt64SetAddArgs nnint64;
};

typedef StorageDelta StorageDeltaVec<>;

}
