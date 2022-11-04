#include <iostream>
#include <string>
#include <map>
#include <fstream>
#include <vector>

struct Name {
    std::string name;

    Name(std::string& line) {
        char left = line.at(0);
        char right = line.at(1);

        this->name = std::string(1, left) + right;
    }

    static Name search_pattern(char left, char right) {
        std::string line = std::string(1, left) + right;
        return Name(line);
    }

    bool operator<(const Name& n) const {
        return this->name < n.name;
    }
};

struct Rule {
    char left;
    char right;
    char insert;

    std::vector<Rule*> next_rules;
    std::map<char, unsigned long long> next_values;

    Rule(std::string& line);
    std::string activate() const;
    void add_next_rule(Rule* next_rule);
};

std::string Rule::activate() const {
    return std::string(1,left) + insert + right;
}

void Rule::add_next_rule(Rule* next_rule) {
    this->next_rules.push_back(next_rule);
}

Rule::Rule(std::string& line) {
    this->left = line.at(0);
    this->right = line.at(1);
    this->insert = line.at(line.size()-1);
}

std::pair<std::string, std::map<Name, Rule>> parse_input(bool test) {
    std::string path = "../input/year2021/";
    if (test) {
        path += "test14.txt";
    } else {
        path += "day14.txt";
    }

    std::ifstream infile(path);
    if (!infile) {
        std::cout << "Could not find file '" << path << "'" << std::endl;
        return {};
    }

    std::string poly_template;
    std::getline(infile, poly_template);

    std::string line;
    std::getline(infile, line); // Empty line

    std::map<Name, Rule> rules;
    while (infile) {
        std::getline(infile, line);

        Rule r(line);
        Name n(line);

        rules.insert(std::pair<Name, Rule>(n, r));
    }

    return std::pair<std::string, std::map<Name, Rule>>(poly_template, rules);
}

constexpr int steps = 5;
// Brute force 5 steps
std::string brute_force(Rule& r, std::map<Name, Rule>& rules) {
    std::string result = std::string(1, r.left) + r.right;

    for (int i = 0; i < steps; i++) {
        std::string new_result;
        for (int j = 0; j < result.size()-1; j++) {
            new_result += rules
                    .find(Name::search_pattern(result.at(j), result.at(j+1)))
                    ->second.activate();
        }
        result = new_result;
    }

    return result;
}

void determine_next_rules(std::map<Name, Rule>& rules) {
    for (auto i = rules.begin(); i != rules.end(); i++) {
        std::string activated_and_run = brute_force(i->second, rules);

        for (int j = 0; j < activated_and_run.size()-1; j++) {
            i->second.add_next_rule(
        &rules.find(Name::search_pattern(
                activated_and_run.at(j), activated_and_run.at(j+1))
                )->second
            );
        }
    }
}

std::map<char, unsigned long long> get_quantities(std::string str) {
    std::map<char, unsigned long long> quantities;

    for (char c : str) {
        unsigned long long num = 1;
        if (quantities.find(c) != quantities.end()) {
            num = quantities.find(c)->second + 1;
        }
        quantities[c] = num;
    }

    return quantities;
}

void add_char(char c, std::map<char, unsigned long long>& m) {
    unsigned long long num = 1;
    if (m.find(c) != m.end()) {
        num = m.find(c)->second + 1;
    }
    m[c] = num;
}

void post_order(unsigned int depth_to_go, unsigned int step, std::map<char, unsigned long long>& q, Rule* r) {
    add_char(r->insert, q);
    if (step == depth_to_go) {
        return;
    }

    for (int i = 0; i < r->next_rules.size(); i++) {
        post_order(depth_to_go, step+5, q, r->next_rules.at(i));
    }
}

unsigned long long calculate_score(std::map<char, unsigned long long>& m) {
    unsigned long long max = 0;
    unsigned long long min = ULLONG_MAX;

    for (auto i : m) {
        if (i.second > max) {
            max = i.second;
        } else if (i.second < min) {
            min = i.second;
        }
    }

    return max - min;
}

unsigned long long part(std::string poly_template, unsigned int steps_to_go, std::map<Name, Rule>& rules) {
    std::map<char, unsigned long long> quantities = get_quantities(poly_template);

    for (int i = 0; i < poly_template.size()-1; i++) {
        std::cout << "Pair: " << poly_template.at(i) << poly_template.at(i+1) << std::endl;
        Rule r = rules.find(Name::search_pattern(poly_template.at(i), poly_template.at(i+1)))->second;

        post_order(steps_to_go, 1, quantities, &r);
    }

    return calculate_score(quantities);
}

int main() {
    std::pair<std::string, std::map<Name, Rule>> input = parse_input(false);
    determine_next_rules(input.second);

    unsigned long long result = part(input.first, 10, input.second);

    std::cout << "The result is " << result << std::endl;
}