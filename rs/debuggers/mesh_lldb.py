# rust-lldb target/debug/examples/debug_target
# (lldb) command script import debuggers/mesh_lldb.py
import lldb


def mesh_summary(valobj, _internal_dict):
    """One-line view of a debug_target::Mesh"""
    name = valobj.GetChildMemberWithName("name").GetSummary()  # std printer
    nodes = valobj.GetChildMemberWithName("nodes").GetNumChildren()  # idem
    return f"Mesh {name} ({nodes} nodes)"


def __lldb_init_module(debugger, _internal_dict):
    # Registered for the Rust type name, called whenever a Mesh is displayed
    debugger.HandleCommand(
        "type summary add -F mesh_lldb.mesh_summary debug_target::Mesh")
