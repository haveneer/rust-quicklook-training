#include <iostream>
#include <string>
#include <cassert>

class IGateKeeper {
public:
  virtual bool CanEnter(std::string id, int age) = 0;
};

class MajorityGateKeeper : public IGateKeeper {
public:
  bool CanEnter(std::string id, int age) override {
    std::puts("MajorityGateKeeper");
    return age >= 18;
  }
};

class VipGateKeeper : public MajorityGateKeeper {
public:
  bool CanEnter(std::string id, int age) override {
    // Do the majority test and check if the id is VIP.
    std::puts("VipGateKeeper");
    return MajorityGateKeeper::CanEnter(id, age) && (id == "Chuck Norris");
  }
};

class DrunkGateKeeper : public VipGateKeeper {
public:
  bool CanEnter(std::string id, int age) override {
    std::puts("DrunkGateKeeper");
    return true;
  }
};

int main() {
  VipGateKeeper *gk = new DrunkGateKeeper();
  bool canEnter = gk->CanEnter("Miley Cyrus", 16);
  assert(canEnter == true);
}