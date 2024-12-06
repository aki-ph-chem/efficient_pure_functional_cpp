#include <iostream>
#include <numeric>
#include <vector>
#include <cmath>

auto sum(const std::vector<double>& v) -> double {
    return std::accumulate(v.begin(), v.end(), 0, std::plus<double>());
}

auto mean(const std::vector<double>& v) -> double {
    return sum(v) / static_cast<double>(v.size());
}

// vをコピーする
auto squared(std::vector<double> v) -> std::vector<double> {
    std::transform(v.begin(), v.end(), v.begin(), [](double d) {return d * d;});
    return std::move(v);
}

// rmsを計算する
auto rms(std::vector<double> v) -> double {
    return std::sqrt(mean(squared(v)));
}

auto main(void) -> int {
    auto res = rms(std::vector<double>{1,2,3});
    std::cout << "res = " << res << "\n";

    return 0;
}
