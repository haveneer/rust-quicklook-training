#include <version>
#if !defined(__cpp_concepts)
//#region [Feature check]
#if __has_include("unsupported_features.hpp")
#include "unsupported_features.hpp"
REPORT_FEATURES({STR(__cpp_concepts)});
#else
#error "Unsupported feature"
#endif
//#endregion
#else

//#region [Collapse all]
// Inspired from https://mcla.ug/blog/cpp20-concepts-are-not-like-rust-traits.html
#include <string>
#include <type_traits>

template <typename T, typename U>
concept same_as = std::is_same<T, U>::value;
//#endregion

template <typename T>
concept Stringable = requires(T a) {
  { a.stringify() } -> same_as<std::string>;
};

class Cat {
public:
  std::string stringify() { return "meow"; }
  void pet() {}
};

template <Stringable T> void f(T a) { a.pet(); }

int main() {
  f(Cat{});
}

#endif