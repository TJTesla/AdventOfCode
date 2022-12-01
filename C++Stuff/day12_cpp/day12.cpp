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

enum File {
	TEST1,
	TEST2,
	TEST3,
	INPUT,
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

		caves->insert(std::pair<std::string, Cave>(str, Cave(str, type)));
	}
}

std::map<std::string, Cave> parse_input(File f) {
	std::string file = "../input/";
	switch (f) {
		case TEST1:
			file += "test12_1.txt";
			break;
		case TEST2:
			file += "test12_2.txt";
			break;
		case TEST3:
			file += "test12_3.txt";
			break;
		case INPUT:
			file += "day12.txt";
	}

	std::map<std::string, Cave> caves;

	std::ifstream infile(file);
	if (!infile) {
		std::cout << "Could not find file '" << file << "'" << std::endl;
		return {};
	}
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

struct Tree {
	Tree* parent;
	Cave* c;
	std::vector<Tree*> followers;
	Tree(Tree* parent, Cave* c) { this->parent = parent; this->c = c; }
};

void free_tree(Tree* t, bool begin);
unsigned long traversal(Tree* t);

bool check_if_used(Tree* parent, Cave* c, int times_seen, std::set<std::string> occurences, bool doubly_used) {
	// Check for End or Start has to be happened earlier
	if (c->get_type() == Big || c->get_type() == End || parent == nullptr) {
		return false;
	}
	if (parent->c->get_type() == Small) {
		auto result =  occurences.insert(parent->c->get_name());
		if (!result.second) {
			if (doubly_used) {
				return true;
			} else {
				doubly_used = true;
			}
		}
	}

	return check_if_used(parent->parent, c, times_seen, occurences, doubly_used);
}

void print_path(Tree* parent) {
	std::string path;
	Tree* current = parent;
	while (current != nullptr) {
		path = "-" + current->c->get_name() + path;
		current = current->parent;
	}

	std::cout << path << std::endl;
}

int main() {
	std::map<std::string, Cave> caves = parse_input(INPUT);
	Cave start = caves.find("start")->second;
	start.visit();

	unsigned long counter = 0;

	Tree tree(nullptr, &start);
	std::vector<Tree*> trees;
	trees.push_back(&tree);


	int iterations = 0;

	while (!trees.empty()) {
		Tree* current = trees.front();
		//current->c->visit();
		if (current->c->get_type() == End) {
			//print_path(current);
			trees.erase(trees.begin());
			continue;
		}
		for (int j = 0; j < current->c->get_all_connections().size(); j++) {
			Cave* neighbouring_cave = current->c->get_all_connections().at(j);
			if (neighbouring_cave->get_type() == Start) {
				continue;
			}
			if (check_if_used(current, neighbouring_cave, 0, {neighbouring_cave->get_name()}, false)) {
				continue;
			}
			trees.push_back( new Tree(current, neighbouring_cave) );
			current->followers.push_back(trees.at(trees.size()-1));
		}
		trees.erase(trees.begin());
		iterations += 1;
	}

	counter = traversal(&tree);

	std::cout << "There are " << counter << " possible ways" << std::endl;

	free_tree(&tree, true);

	return 0;
}

unsigned long traversal(Tree* t) {
	if (t->c->get_type() == End) {
		return 1;
	}

	unsigned long counter = 0;
	for (int i = 0; i < t->followers.size(); i++) {
		counter += traversal(t->followers.at(i));
	}

	return counter;
}

void free_tree(Tree* t, bool begin) {
	for (int i = t->followers.size()-1; i >= 0; i--) {
		free_tree(t->followers.at(i), false);
		t->followers.erase(t->followers.begin()+i);
	}
	if (!begin) {
		delete t;
	}
}