// Prints how libstdc++ orders std::unordered_map<std::string, int>, for the port's
// emulation of it (uv_core::std_unordered). The reference's spelling suggestions and
// wildcard imports depend on this order.
#include <cstdint>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

namespace std::__detail {
extern const unsigned long __prime_list[];
}

using Map = std::unordered_map<std::string, int>;

std::string Key(std::size_t i) {
  static const std::string alphabet = "abcdefghijklmnopqrstuvwxyz0123456789_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const std::size_t mixed = (i * 2654435761ull) % 1000003ull;
  // Lengths from 1 to 24, so that every tail length of the hash is exercised.
  std::string key = alphabet.substr(i % 7, (i % 24) + 1 > 3 ? (i % 24) - 2 : 0);
  return key + std::to_string(mixed);
}

void Print(const char* tag, std::size_t id, const Map& map) {
  std::cout << tag << ' ' << id << ' ' << map.bucket_count();
  for (const auto& entry : map) std::cout << ' ' << entry.first;
  std::cout << '\n';
}

int main() {
  std::cout << "PRIMES";
  for (int i = 0; i < 120; ++i) std::cout << ' ' << std::__detail::__prime_list[i];
  std::cout << '\n';
  for (std::size_t i = 0; i < 40; ++i) {
    const std::string key = Key(i * 13 + 1);
    std::cout << "H " << key << ' ' << std::hash<std::string>{}(key) << '\n';
  }
  std::cout << "H - " << std::hash<std::string>{}(std::string()) << '\n';
  const std::vector<std::size_t> sizes = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 20, 28, 29,
                                         30, 31, 40, 58, 59, 60, 61, 62, 100, 126, 127, 128, 129, 200, 256, 257,
                                         258, 300, 541, 542, 700, 1109, 1110, 1500, 2357, 2358, 3000};
  for (const std::size_t n : sizes) {
    Map map;
    for (std::size_t i = 0; i < n; ++i) map.emplace(Key(i), 0);
    Print("N", n, map);
  }
  // Inserting an existing key, and assigning through operator[], leave the order alone.
  {
    Map map;
    for (std::size_t i = 0; i < 20; ++i) map.emplace(Key(i), 0);
    for (std::size_t i = 0; i < 20; i += 3) map.emplace(Key(i), 1);
    for (std::size_t i = 15; i < 25; ++i) map[Key(i)] = 2;
    Print("R", 0, map);
  }
  // A copy keeps the order and the bucket count, and grows as the original would.
  for (const std::size_t n : {3u, 10u, 13u, 14u, 29u, 30u, 59u, 100u}) {
    Map base;
    for (std::size_t i = 0; i < n; ++i) base.emplace(Key(i), 0);
    Map copy = base;
    Print("C", n, copy);
    for (std::size_t i = 0; i < 9; ++i) copy.emplace(Key(1000 + i), 0);
    Print("D", n, copy);
    // Copy assignment over a map that already has elements.
    Map target;
    for (std::size_t i = 0; i < 5; ++i) target.emplace(Key(2000 + i), 0);
    target = base;
    Print("A", n, target);
    for (std::size_t i = 0; i < 9; ++i) target.emplace(Key(3000 + i), 0);
    Print("B", n, target);
    Map big;
    for (std::size_t i = 0; i < 200; ++i) big.emplace(Key(4000 + i), 0);
    big = base;
    Print("G", n, big);
    for (std::size_t i = 0; i < 9; ++i) big.emplace(Key(5000 + i), 0);
    Print("I", n, big);
  }
  return 0;
}
