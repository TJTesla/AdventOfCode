#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <fstream>
#include <sstream>
#include <locale>

struct Line {
    std::string left;
    std::string right;

    Line(std::string l, std::string r) {
        this->left = l;
        this->right = r;
    }
};

Line split(std::string str, char del) {
    int del_index = -1;
    for (int i = 0; i < str.length(); i++) {
        if (str.at(i) == del) {
            del_index = i;
            break;
        }
    }

    Line l(str.substr(0, del_index), str.substr(del_index+1));

    return l;
}

enum CaveType {
    Big,
    Small,
    Start,
    End
};

class Cave {
protected:
    CaveType type;
    std::string name;
    std::vector<Cave*> connections;

    bool visited;
public:
    Cave(std::string name, CaveType type);
    void add_connection(Cave* c);
    std::vector<Cave*> get_all_connections();
    std::string get_name();

    void visit();
    bool is_visited();
    CaveType get_type() { return this->type; }
};

Cave::Cave(std::string name, CaveType type) {
    this->name = name;
    this->type = type;
    this->visited = false;
}

void Cave::visit() {
    if (this->type == Big) {
        return;
    }

    this->visited = true;
}

bool Cave::is_visited() {
    return this->visited;
}

void Cave::add_connection(Cave* c) {
    this->connections.push_back(c);
}

std::vector<Cave*> Cave::get_all_connections() {
    return this->connections;
}

std::string Cave::get_name() {
    return this->name;
}

bool is_lower(std::string str) {
    for (int i = 0; i < str.length(); i++) {
        if (!std::islower(str.at(i))) {
            return false;
        }
    }

    return true;
}

void create_cave(std::map<std::string, Cave>* caves, std::string str) {
    if (caves->find(str) == caves->end()) {
        CaveType type;
        if (str == "start") {
            type = Start;
        } else if (str == "end") {
            type = End;
        } else if (is_lower(str)) {
            type = Small;
        } else {
            type = Big;
        }

        if (type != Big) {
            caves->insert(std::pair<std::string, Cave>(str, Cave(str, type)));
        } else {
            caves->insert(std::pair<std::string, Cave>(str, Cave(str, type)));
        }
    }
}

std::map<std::string, Cave> parse_input(bool test) {
    std::string file;
    if (test) {
        file = "../input/year2021/test12.txt";
    } else {
        file = "..input/year2021/day12.txt";
    }

    std::map<std::string, Cave> caves;

    std::ifstream infile(file);
    std::string line;
    while (std::getline(infile, line)) {
        Line l = split(line, '-');

        create_cave(&caves, l.left);
        create_cave(&caves, l.right);

        caves.find(l.left)->second.add_connection(&caves.find(l.right)->second);
        caves.find(l.right)->second.add_connection(&caves.find(l.left)->second);
    }

    return caves;
}

void append(std::vector<Cave*>* list, std::vector<Cave*>& neighbours) {
    for (int i = 0; i < neighbours.size(); i++) {
        
        list->push_back(neighbours.at(i));
    }
}

int main() {
    std::map<std::string, Cave> caves = parse_input(true);
    Cave start = caves.find("start")->second;
    start.visit();

    std::vector<Cave*> list;
    list.push_back(&start);

    unsigned long counter = 0;
    std::string way = "";

    while (!list.empty()) {
        Cave* current = list.front();
        way += current->get_name();
        std::vector<Cave*> neighbours = current->get_all_connections();

        for (int i = 0; i < neighbours.size(); i++) {
            Cave* c = neighbours.at(i);
            if (c->get_type() == End) {
                counter += 1;
                std::cout << way << std::endl;
                way = "";
            } else if (!c->is_visited() && c->get_type() != Start) {
                c->visit();
                list.push_back(c);
            }
        }

        list.erase(list.begin());
    }
    
    std::cout << "There are " << counter << " possible ways" << std::endl;

    return 0;
}