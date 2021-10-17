struct Object {
  [[nodiscard]] constexpr int method() const noexcept { return 1; }
};

[[nodiscard]] constexpr int function(const Object &o) noexcept(&Object::method) {
  return o.method();
}

int main() {
  Object o;
  return function(o);
}