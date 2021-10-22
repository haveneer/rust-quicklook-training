//#region [Declarations]
#include <iostream>
#include <memory>
#include <type_traits>
#include <utility>
#include <variant>
//#endregion

struct Empty;
struct Node;

using Tree = std::variant<Empty, Node>;

struct Empty {};
struct Node {
  std::unique_ptr<Tree> left;
  std::unique_ptr<Tree> right;
};

//#region [overload trick]
template <typename... Ts> struct overload : Ts... {
  // overload(Ts... ts) : Ts(ts)... {} // can be replaced by CTAD
  using Ts::operator()...;
};
// Custom Template Argument Deduction Rules
template <typename... Ts> overload(Ts...) -> overload<Ts...>;
//#endregion

int main() {
  Tree tree1{Empty{}};
  Tree tree2{Node{std::make_unique<Tree>(Empty{}),
                  std::make_unique<Tree>(Node{std::make_unique<Tree>(Empty{}),
                                              std::make_unique<Tree>(Empty{})})}};
  std::visit(
      [](const auto &t) {
        using type = std::decay_t<decltype(t)>;
        // clang-format off
        if constexpr (std::is_same_v<type, Empty>) { std::cout << "Empty\n"; }
        if constexpr (std::is_same_v<type, Node>) { std::cout << "Node\n"; }
        // clang-format on
      },
      tree1);

  std::visit(overload{[](const Empty &) { std::cout << "Empty\n"; },
                      [](const Node &) { std::cout << "Node\n"; },
                      [](auto x) { /* default case (useless here) */ }},
             tree2);
}