#include "field.hpp"

namespace sim {

Field::Field(size_t size) : m_values(size) {}

const std::string &Field::name() const { return m_name; }

void Field::set_name(const std::string &name) { m_name = name; }

size_t Field::size() const { return m_values.size(); }

const std::vector<double> &Field::values() const { return m_values; }

std::vector<double> &Field::values_mut() { return m_values; }

} // namespace sim
