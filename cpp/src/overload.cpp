struct Interface {
  virtual void f() = 0;
};

struct A : Interface {
  void f() override {}
};

struct B : Interface  {
  void f() override {}
};

void f(A a) {}
void f(B b) {}
void f(double) {}

int main() {
  A a;
  B b;

  a.f();
  f(a);

  b.f();
  f(b);
  
  f(3.14);
}