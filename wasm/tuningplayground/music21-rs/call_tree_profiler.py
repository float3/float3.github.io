#!/usr/bin/env python3

import sys
import json
import threading
import os



class CallNode:
    def __init__(self, name, filename, lineno, parent=None):
        self.name = name
        self.filename = filename
        self.lineno = lineno
        self.parent = parent
        self.children = []

    def to_dict(self):
        return {
            'name': self.name,
            'filename': self.filename,
            'lineno': self.lineno,
            'children': [child.to_dict() for child in self.children]
        }

class CallTreeProfiler:
    def __init__(self, project_root):
        self.root = CallNode("root", "", 0)
        self.current_node = self.root
        self.lock = threading.Lock()
        self.project_root = os.path.abspath(project_root)

    def is_internal_frame(self, frame):
        """
        Determines if a frame is internal based on its filename.
        """
        filename = frame.f_code.co_filename
        name = frame.f_code.co_name
        # Define exclusion criteria
        if '<frozen' in filename or '<string>' in filename:
            return True
        if 'site-packages' in filename:
            return True
        if 'Lib' in filename and ('site-packages' not in filename):
            return True
        if filename.startswith(os.path.join(sys.prefix, 'Lib')):
            return True
        if not os.path.abspath(filename).startswith(self.project_root):
            return True
        if name == "<genexpr>":
            return True
        
        return False

    def profile(self, frame, event, arg):
        if event not in ('call', 'return'):
            return
        code = frame.f_code
        func_name = code.co_name
        filename = frame.f_code.co_filename
        lineno = frame.f_lineno

        if self.is_internal_frame(frame):
            return  # Skip internal frames

        with self.lock:
            if event == 'call':
                new_node = CallNode(func_name, filename, lineno, parent=self.current_node)
                self.current_node.children.append(new_node)
                self.current_node = new_node
            elif event == 'return':
                if self.current_node.parent is not None:
                    self.current_node = self.current_node.parent

    def save_json(self, filename):
        with open(filename, 'w') as f:
            json.dump(self.root.to_dict(), f, indent=2)

    def save_tree(self, filename):
        def write_node(node, indent=0, file=None):
            prefix = "  " * indent + "- " if indent > 0 else ""
            file.write(f"{prefix}{node.name}() at {node.filename}:{node.lineno}\n")
            for child in node.children:
                write_node(child, indent + 1, file)

        with open(filename, 'w') as f:
            f.write("Call Stack Tree:\n")
            for child in self.root.children:
                write_node(child, indent=1, file=f)

