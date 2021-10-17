#include <string>

struct Object {
  explicit Object(std::string s) noexcept : attr(std::move(s)) {}
  [[nodiscard]] constexpr auto method() const noexcept { return 0; }
  const std::string attr;
};

[[nodiscard]] constexpr auto function(const Object &o) noexcept(noexcept(&Object::method)) {
  return o.method();
}

int main() {
  Object o("String");
  return function(o);
}