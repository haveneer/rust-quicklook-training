template <typename T> auto foo(T arg) -> T { return arg + 1; }

int main() {
  foo(1);
  foo(3.14);
  foo("hello");
}