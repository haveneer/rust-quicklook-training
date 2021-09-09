//#region [Headers]
#include <cstring>
#include <exception>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
using namespace std::string_literals;
//#endregion

class Connection { // /!\ NOT RAII
private:
  std::unique_ptr<char[]> m_addr;

public:
  void connect(char addr[]) { m_addr.reset(addr); }
  void connect(std::string addr) {
    m_addr.reset(new char[addr.size() + 1]);
    std::strcpy(m_addr.get(), addr.data());
  }
  friend std::ostream &operator<<(std::ostream &o, const Connection &data) {
    return o << data.m_addr;
  }

public:
  void dispose() { m_addr.reset(); }
};

void do_something(Connection &) {
  //  throw std::runtime_error("f call has failed");
}

int main(int argc, char *argv[]) {
  Connection conn;

  if (argc > 1) {
    conn.connect(argv[1]);
  } else {
    conn.connect("Default value"s);
  }

  std::cout << "connection before = " << conn << std::endl;

  try {
    do_something(conn);
  } catch (std::exception &e) {
    std::cout << "An exception has been thrown\n";
    conn.dispose(); // may be an illegal free
  }

  std::cout << "connection after = " << conn << std::endl; // CWE416 ?
  conn.dispose();                                          // double free ?
}