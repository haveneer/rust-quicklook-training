//#region [Collapse all]
#include <iostream>
#include <vector>
//#endregion

class Rectangle {
public:
  Rectangle() = default;
  Rectangle(int height, int width) : m_height(height), m_width(width) {}
  virtual int area() const { return m_height * m_width; }

private:
  int m_height = {}, m_width = {};
};

class Square : public Rectangle {
public:
  Square() = default;
  Square(int side) : m_side(side) {}
  int area() const override { return m_side * m_side; }

private:
  int m_side = {};
};

int main() {
  std::vector<Rectangle> v;
  v.push_back(Rectangle{2, 5});
  v.push_back(Square{3});

  for (auto &&x : v) {
    std::cout << x.area() << "\n";
  }
}