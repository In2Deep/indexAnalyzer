
#!/usr/bin/env python3
"""
🧠 AI's External Brain Cells 🧠
--------------------------------
Because even smart AIs need a memory upgrade sometimes!

Ever had your AI assistant say: "As I mentioned earlier..." when they definitely DIDN'T?
Or watched them confidently hallucinate your codebase like they're on a weekend trip?
This tool is the digital equivalent of sticky notes for your AI.

USAGE:
  brain_cells remember ~/my-project   # Index code (like drinking coffee before work)
  brain_cells refresh app/models.py   # Update specific files (when you made them better!)
  brain_cells recall class UserModel   # Find specific code (where did I put those keys?)
  brain_cells status                  # Check memory status (is anyone home?)
  brain_cells forget                  # Clear indexed data (for that Monday morning feeling)

WARNING: Side effects may include fewer hallucinations, less repetition, 
and an AI that actually remembers what your code does!
"""
import os
import ast
import json
import sys
import logging
import argparse
import asyncio
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Set, Tuple, Optional, Union

try:
    import redis.asyncio as redis
    REDIS_PY_AVAILABLE = True
except ImportError:
    REDIS_PY_AVAILABLE = False
    print("💔 Redis package missing - your AI will continue to have amnesia!")

try:
    import pathspec
    PATHSPEC_AVAILABLE = True
except ImportError:
    PATHSPEC_AVAILABLE = False
    print("🙈 pathspec missing - I'll just ignore your .gitignore... like everyone else!")

# Places we won't peek (I respect privacy!)
SKIP_DIRS = {'.logs', '.venv', '.git', '__pycache__', 'node_modules', 'build', 'dist'}
REDIS_URL = os.environ.get("REDIS_URL", "redis://127.0.0.1:6379/0")
VERSION = "1.0.0"

######################
# Memory Maintenance #
######################

def setup_logging(log_dir: Path) -> logging.Logger:
    """Configure the diary where we write down our thoughts."""
    log_dir.mkdir(parents=True, exist_ok=True)
    log_file = log_dir / "brain_activity.log"
    
    logger = logging.getLogger("AIMemory")
    logger.setLevel(logging.INFO)
    
    if logger.hasHandlers():
        logger.handlers.clear()  # Out with the old thoughts
        
    file_handler = logging.FileHandler(log_file, encoding='utf-8', mode='w')
    file_handler.setLevel(logging.INFO)
    file_formatter = logging.Formatter('%(asctime)s - %(levelname)s - %(message)s')
    file_handler.setFormatter(file_formatter)
    logger.addHandler(file_handler)
    
    console_handler = logging.StreamHandler(sys.stderr)
    console_handler.setLevel(logging.INFO)
    console_formatter = logging.Formatter('%(levelname)s: %(message)s')
    console_handler.setFormatter(console_formatter)
    logger.addHandler(console_handler)
    
    return logger

def load_gitignore_patterns(base_dir: Path) -> Optional[Any]:
    """Load gitignore, because some secrets are better left forgotten."""
    gitignore_path = base_dir / ".gitignore"
    if PATHSPEC_AVAILABLE and gitignore_path.exists():
        with open(gitignore_path, "r") as f:
            patterns = f.read().splitlines()
        return pathspec.PathSpec.from_lines("gitwildmatch", patterns)
    return None

def is_ignored(path: Path, base_dir: Path, gitignore_spec) -> bool:
    """Check if a file wants to be left alone. We respect boundaries."""
    if gitignore_spec is None:
        return False
    rel_path = str(path.relative_to(base_dir))
    return gitignore_spec.match_file(rel_path)

def collect_python_files(app_dir: Path, gitignore_spec, specific_files: Optional[List[str]] = None) -> List[Path]:
    """Find all Python files like they're Easter eggs."""
    if specific_files:
        return [app_dir / file_path if not os.path.isabs(file_path) else Path(file_path) 
                for file_path in specific_files 
                if (app_dir / file_path).exists() and (app_dir / file_path).is_file()]
    
    python_files = []
    for root, dirs, files in os.walk(app_dir, topdown=True):
        # Skip directories that are basically "nothing to see here"
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        
        # Respect the "do not disturb" signs
        if gitignore_spec:
            dirs[:] = [d for d in dirs if not is_ignored(Path(root) / d, app_dir, gitignore_spec)]
            
        root_path = Path(root)
        for file in files:
            if file.endswith('.py') and (not gitignore_spec or not is_ignored(root_path / file, app_dir, gitignore_spec)):
                python_files.append(root_path / file)
                
    return python_files

##########################
# Code Analyzing Wizards #
##########################

def _get_signature(node: ast.FunctionDef, source_lines: List[str]) -> str:
    """Extract a function's autograph - like I'm a code celebrity stalker."""
    try:
        if node.body:
            first_body_line = node.body[0].lineno - 1
            sig_end_line = node.lineno
            current_line = first_body_line
            
            # Hunt for the colon like it's buried treasure
            while current_line >= node.lineno - 1:
                line_content = source_lines[current_line].rstrip()
                if line_content.endswith(':'):
                    sig_end_line = current_line + 1
                    break
                if current_line > node.lineno-1 and ':' in source_lines[current_line-1]:
                    sig_end_line = current_line
                    break
                current_line -= 1
            else:
                sig_end_line = node.lineno
            
            # Extract the signature with surgical precision
            signature_lines = source_lines[node.lineno-1 : sig_end_line]
            sig_str = " ".join(line.strip() for line in signature_lines).split('#')[0].strip()
            
            if sig_str.endswith(':'):
                sig_str = sig_str[:-1].strip()  # Off with the colon!
                
            sig_str = sig_str.replace('\n', ' ').strip()
            return sig_str if sig_str else f"def {node.name}(...):"
        else:
            return ast.unparse(node)
    except Exception:
        # When all else fails, just guess
        return f"def {node.name}(...)"

def extract_code_info(file_path: Path, source_lines: List[str], base_dir: Path) -> List[Dict]:
    """Perform a deep code investigation, CSI-style."""
    entities = []
    rel_path = file_path.relative_to(base_dir).as_posix()
    
    try:
        # Turn code into a tree - time for the AST-rological reading
        tree = ast.parse("\n".join(source_lines))
        
        # Add parent references - because everyone needs to know their daddy
        for node in ast.walk(tree):
            for child in ast.iter_child_nodes(node):
                child.parent = node  # type: ignore
    except SyntaxError:
        # Bad syntax? Not my problem!
        return []
    except Exception:
        # Mystery error? Also not my problem!
        return []
    
    # Walk the tree like we're on a code safari
    for node in ast.walk(tree):
        entity_data = None
        entity_type = None
        entity_name = None
        parent_name = None
        
        try:
            if isinstance(node, ast.FunctionDef):
                parent = getattr(node, 'parent', None)
                
                # Is it a method or a lone wolf function?
                if isinstance(parent, ast.ClassDef):
                    entity_type = "method"
                    parent_name = parent.name
                elif isinstance(parent, ast.Module):
                    entity_type = "function"
                else:
                    continue
                    
                entity_name = node.name
                entity_data = {
                    "signature": _get_signature(node, source_lines),
                    "docstring": ast.get_docstring(node) or "",
                    "line_start": node.lineno,
                    "line_end": getattr(node, 'end_lineno', node.lineno)
                }
                
                if parent_name:
                    entity_data["parent_class"] = parent_name
                    
            elif isinstance(node, ast.ClassDef):
                parent = getattr(node, 'parent', None)
                
                # Only care about module-level classes, not weird nested ones
                if not isinstance(parent, ast.Module):
                    continue
                    
                entity_type = "class"
                entity_name = node.name
                
                # Who's your daddy? (class inheritance)
                bases = []
                for base in node.bases:
                    try:
                        base_repr = ast.unparse(base)
                    except AttributeError:
                        base_repr = getattr(base, 'id', '<fancy_parent_i_cant_understand>')
                    except Exception:
                        base_repr = '<parent_issues>'
                    bases.append(base_repr)
                    
                entity_data = {
                    "bases": bases,
                    "docstring": ast.get_docstring(node) or "",
                    "line_start": node.lineno,
                    "line_end": getattr(node, 'end_lineno', node.lineno)
                }
                
            elif isinstance(node, ast.Assign):
                parent = getattr(node, 'parent', None)
                is_module_level = isinstance(parent, ast.Module)
                is_class_level = isinstance(parent, ast.ClassDef)
                
                # We only care about class or module variables
                if is_module_level or is_class_level:
                    for target in node.targets:
                        if isinstance(target, ast.Name):
                            entity_type = "variable"
                            entity_name = target.id
                            
                            # What's inside the box?
                            value_repr = "<something_complicated>"
                            try:
                                if isinstance(node.value, ast.Constant):
                                    value_repr = repr(node.value.value)
                                elif isinstance(node.value, ast.List):
                                    value_repr = "[lots_of_stuff]"
                                elif isinstance(node.value, ast.Dict):
                                    value_repr = "{key: value, ...}"
                            except Exception:
                                pass
                                
                            entity_data = {
                                "value_repr": value_repr,
                                "line_start": node.lineno,
                                "line_end": getattr(node, 'end_lineno', node.lineno)
                            }
                            
                            if is_class_level:
                                entity_data["parent_class"] = parent.name  # type: ignore
                                
                            break
                            
        except Exception:
            # When the going gets tough, the tough skip and move on
            continue
            
        # If we found something interesting, add it to our collection
        if entity_type and entity_name and entity_data:
            base_entity = {
                "entity_type": entity_type,
                "file_path": rel_path,
                "name": entity_name,
                **entity_data
            }
            entities.append(base_entity)
            
    return entities

#######################
# Redis Memory Palace #
#######################

async def index_file_contents(file_path: Path, base_dir: Path, key_prefix: str, 
                             redis_client: Optional[redis.Redis] = None, 
                             logger: Optional[logging.Logger] = None) -> bool:
    """Store file content in our memory palace (aka Redis)."""
    rel_path = file_path.relative_to(base_dir).as_posix()
    
    try:
        # Read the file like it's a gripping novel
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            
        file_data = {
            "path": rel_path,
            "content": content,
            "size": len(content),  # Size matters, apparently
            "last_modified": os.path.getmtime(file_path)
        }
        
        file_key = f"{key_prefix}:files:{rel_path}"
        
        if redis_client:
            # Store in Redis - your memories are safe with us
            await redis_client.set(file_key, json.dumps(file_data, ensure_ascii=False))
            await redis_client.sadd(f"{key_prefix}:file_index", rel_path)
            return True
        else:
            if logger:
                logger.error("Redis client missing - I have nowhere to store my memories!")
            return False
            
    except Exception as e:
        if logger:
            logger.error(f"Failed to memorize {rel_path}: {str(e)}")
        return False

async def store_code_entity_by_type(entities: List[Dict], key_prefix: str, 
                                  redis_client: Optional[redis.Redis] = None, 
                                  logger: Optional[logging.Logger] = None) -> int:
    """File code facts in our organized brain drawer system."""
    if not redis_client:
        if logger:
            logger.error("Redis client missing - brain drawers unavailable!")
        return 0
        
    # Sort entities by type like an obsessive organizer
    entities_by_type = {}
    for entity in entities:
        entity_type = entity["entity_type"]
        if entity_type not in entities_by_type:
            entities_by_type[entity_type] = []
        entities_by_type[entity_type].append(entity)
    
    total_stored = 0
    
    # Store each type in its own drawer
    for entity_type, type_entities in entities_by_type.items():
        type_key = f"{key_prefix}:{entity_type}s"  # Classes → Glasses (just kidding)
        
        pipeline = redis_client.pipeline()
        for entity in type_entities:
            file_path = entity["file_path"]
            name = entity["name"]
            
            # Create a fancy ID card for this entity
            if entity_type == "method":
                parent_class = entity.get("parent_class", "unknown")
                entity_id = f"{file_path}:{parent_class}.{name}"
            else:
                entity_id = f"{file_path}:{name}"
            
            # Store in the filing cabinet
            pipeline.hset(type_key, entity_id, json.dumps(entity, ensure_ascii=False))
            
            # Add to the card catalog for easy searching
            pipeline.sadd(f"{key_prefix}:search_index:{entity_type}:{name}", entity_id)
            
            # Cross-reference with the file it came from
            pipeline.sadd(f"{key_prefix}:file_entities:{file_path}", f"{entity_type}:{entity_id}")
            
            total_stored += 1
            
        await pipeline.execute()
    
    return total_stored

async def store_project_metadata(app_dir: Path, key_prefix: str, 
                               redis_client: Optional[redis.Redis] = None,
                               files_processed: int = 0,
                               entities_processed: int = 0,
                               partial_update: bool = False) -> bool:
    """Save the 'About This Brain' information."""
    if not redis_client:
        return False
        
    # Get any existing brain scan results
    existing_metadata = {}
    try:
        existing_json = await redis_client.get(f"{key_prefix}:metadata")
        if existing_json:
            existing_metadata = json.loads(existing_json)
    except:
        pass
    
    # What time is it? Time to get a watch! (ba-dum-tss)
    current_time = time.time()
    timestamp = datetime.fromtimestamp(current_time).isoformat()
    
    # Create detailed memory report
    metadata = {
        **existing_metadata,
        "name": app_dir.name,
        "path": str(app_dir),
        "last_indexed_at": timestamp,
        "last_indexed_timestamp": current_time,
        "version": VERSION
    }
    
    # For complete brain transplants, update total counts
    if not partial_update:
        metadata["total_files"] = files_processed
        metadata["total_entities"] = entities_processed
    # For brain augmentations, just add the new stuff
    else:
        metadata["total_files"] = (metadata.get("total_files", 0) + files_processed 
                                 - len(set(existing_metadata.get("updated_files", []))))
        metadata["total_entities"] = metadata.get("total_entities", 0) + entities_processed
        
    # Keep track of recent memory modifications
    metadata["updated_files"] = [f.as_posix() if isinstance(f, Path) else f 
                              for f in metadata.get("updated_files", [])][:20]  # Last 20 for goldfish memory
    
    await redis_client.set(f"{key_prefix}:metadata", json.dumps(metadata, ensure_ascii=False))
    return True

async def clear_file_data(file_paths: List[Path], base_dir: Path, key_prefix: str, 
                        redis_client: Optional[redis.Redis] = None, 
                        logger: Optional[logging.Logger] = None) -> int:
    """Perform targeted amnesia for specific files."""
    if not redis_client:
        return 0
        
    count = 0
    pipeline = redis_client.pipeline()
    
    for file_path in file_paths:
        try:
            rel_path = file_path.relative_to(base_dir).as_posix()
            
            # Get the list of things to forget
            entities_key = f"{key_prefix}:file_entities:{rel_path}"
            entity_ids = await redis_client.smembers(entities_key)
            
            if entity_ids:
                for entity_id in entity_ids:
                    # entity_id format: "{entity_type}:{file_path}:{name}"
                    try:
                        entity_parts = entity_id.split(":", 1)
                        entity_type = entity_parts[0]
                        
                        # Delete from its primary home
                        type_key = f"{key_prefix}:{entity_type}s"
                        id_part = entity_parts[1]
                        pipeline.hdel(type_key, id_part)
                        
                        # Remove from the search index too
                        name_parts = id_part.split(":")
                        if len(name_parts) > 1:
                            name = name_parts[-1]
                            if "." in name:  # Handle methods like they're special
                                name = name.split(".")[-1]
                            pipeline.srem(f"{key_prefix}:search_index:{entity_type}:{name}", id_part)
                    except:
                        pass
                        
                # Clear out the whole filing cabinet
                pipeline.delete(entities_key)
                count += 1
                
            # Delete the original file content
            pipeline.delete(f"{key_prefix}:files:{rel_path}")
            
            # Remove from the master index
            pipeline.srem(f"{key_prefix}:file_index", rel_path)
            
        except Exception as e:
            if logger:
                logger.error(f"Failed to clear memory of {file_path}: {e}")
    
    await pipeline.execute()
    return count

####################
# Memory Retrieval #
####################

async def query_code_entity(key_prefix: str, entity_type: str, name: Optional[str] = None, 
                          redis_client: Optional[redis.Redis] = None) -> List[Dict]:
    """Search the brain cells for specific memories."""
    if not redis_client:
        return []
        
    results = []
    
    if entity_type and name:
        # Look for a specific entity by name - like finding that actor whose name is on the tip of your tongue
        search_key = f"{key_prefix}:search_index:{entity_type}:{name}"
        entity_ids = await redis_client.smembers(search_key)
        
        if entity_ids:
            type_key = f"{key_prefix}:{entity_type}s"
            pipeline = redis_client.pipeline()
            
            for entity_id in entity_ids:
                pipeline.hget(type_key, entity_id)
                
            entity_jsons = await pipeline.execute()
            
            for json_str in entity_jsons:
                if json_str:
                    try:
                        entity = json.loads(json_str)
                        results.append(entity)
                    except:
                        pass
    elif entity_type:
        # Get all entities of a type - "show me all the functions!" 
        type_key = f"{key_prefix}:{entity_type}s"
        all_entities = await redis_client.hgetall(type_key)
        
        for entity_json in all_entities.values():
            try:
                entity = json.loads(entity_json)
                results.append(entity)
            except:
                pass
    
    return results

async def get_project_info(key_prefix: str, redis_client: Optional[redis.Redis] = None) -> Dict:
    """Check the brain status - like getting an MRI."""
    info = {
        "indexed": False,
        "metadata": {},
        "counts": {}
    }
    
    if not redis_client:
        return info
    
    # Get the brain metadata
    metadata_json = await redis_client.get(f"{key_prefix}:metadata")
    if metadata_json:
        try:
            info["metadata"] = json.loads(metadata_json)
            info["indexed"] = True
        except:
            pass
    
    # Count all the things we remember
    pipeline = redis_client.pipeline()
    entity_types = ["class", "function", "method", "variable"]
    
    for entity_type in entity_types:
        pipeline.hlen(f"{key_prefix}:{entity_type}s")
    
    # Don't forget to count all the files
    pipeline.scard(f"{key_prefix}:file_index")
    
    counts = await pipeline.execute()
    
    for i, entity_type in enumerate(entity_types):
        info["counts"][entity_type] = counts[i]
    
    info["counts"]["files"] = counts[-1]
    
    return info

#######################
# High-Level Commands #
#######################

async def async_index_project(app_dir: Path, specific_files: Optional[List[str]] = None, 
                            logger: Optional[logging.Logger] = None) -> bool:
    """Create a brain backup of your project."""
    app_dir = app_dir.resolve()
    app_name = app_dir.name
    key_prefix = f"code:{app_name}"
    is_partial = specific_files is not None and len(specific_files) > 0
    
    if logger:
        if is_partial:
            logger.info(f"Upgrading memory with {len(specific_files)} files from {app_name}")
        else:
            logger.info(f"Creating complete memory snapshot of {app_name}")
    
    if not app_dir.is_dir():
        if logger:
            logger.critical(f"Error: Can't find directory {app_dir} - is it imaginary?")
        return False
    
    gitignore_spec = load_gitignore_patterns(app_dir) if PATHSPEC_AVAILABLE else None
    
    if not REDIS_PY_AVAILABLE:
        if logger:
            logger.critical("Error: 'redis' library missing - I need somewhere to store memories!")
        return False
    
    try:
        # Connect to Redis - our external brain storage
        redis_client = redis.from_url(REDIS_URL, decode_responses=True)
        await redis_client.ping()
    except Exception as e:
        if logger:
            logger.critical(f"Error connecting to Redis: {e} - Is your memory bank online?")
        return False
    
    try:
        # For full brain transplants, clear out the old memories
        if not is_partial:
            existing_keys = await redis_client.keys(f"{key_prefix}:*")
            if existing_keys:
                if logger:
                    logger.info(f"Clearing {len(existing_keys)} existing memories - out with the old!")
                await redis_client.delete(*existing_keys)
        
        # Find all the Python files - our precious memories
        python_files = collect_python_files(app_dir, gitignore_spec, specific_files)
        
        if not python_files:
            if logger:
                logger.warning(f"No Python files found in {app_dir} - is this really a Python project?")
            return False
        
        if is_partial and logger:
            logger.info(f"Preparing for targeted memory replacement for {len(python_files)} files")
            
        # For partial updates, clear existing memories of these files
        if is_partial:
            await clear_file_data(python_files, app_dir, key_prefix, redis_client, logger)
        
        # Process each file like it's a precious memory
        total_entities_processed = 0
        for file_path in python_files:
            try:
                # Store the complete file
                await index_file_contents(file_path, app_dir, key_prefix, redis_client, logger)
                
                # Extract and store the important parts
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    source_lines = f.read().splitlines()
                
                entities = extract_code_info(file_path, source_lines, app_dir)
                if entities:
                    stored = await store_code_entity_by_type(entities, key_prefix, redis_client, logger)
                    total_entities_processed += stored
            except Exception as e:
                rel_path_str = file_path.relative_to(app_dir).as_posix()
                if logger:
                    logger.error(f"Failed to process {rel_path_str}: {e} - this memory is corrupted!")
        
        # Save the metadata - brain scan complete!
        await store_project_metadata(
            app_dir, 
            key_prefix, 
            redis_client,
            files_processed=len(python_files),
            entities_processed=total_entities_processed,
            partial_update=is_partial
        )
        
        if logger:
            logger.info(f"Memory upgrade complete! {len(python_files)} files and {total_entities_processed} code snippets stored")
        
        await redis_client.aclose()
        return True
        
    except Exception as e:
        if logger:
            logger.error(f"Memory storage failed: {e}")
        return False

async def async_query(entity_type: str, name: Optional[str] = None, app_dir: Optional[Path] = None, 
                    logger: Optional[logging.Logger] = None) -> List[Dict]:
    """Search for memories about your code."""
    if not REDIS_PY_AVAILABLE:
        if logger:
            logger.critical("Error: 'redis' library missing - can't search empty brain cells!")
        return []
    
    try:
        redis_client = redis.from_url(REDIS_URL, decode_responses=True)
        await redis_client.ping()
        
        # Figure out which project we're thinking about
        if app_dir:
            app_name = app_dir.resolve().name
        else:
            # Find the most recently thought-about project
            projects = []
            pattern = "code:*:metadata"
            metadata_keys = await redis_client.keys(pattern)
            
            for key in metadata_keys:
                app_name = key.split(":")[1]
                metadata_json = await redis_client.get(key)
                if metadata_json:
                    try:
                        metadata = json.loads(metadata_json)
                        projects.append({
                            "name": app_name,
                            "timestamp": metadata.get("last_indexed_timestamp", 0)
                        })
                    except:
                        pass
            
            if not projects:
                if logger:
                    logger.error("No projects found in memory - have you indexed anything?")
                return []
                
            # Sort by most recent - because who remembers old stuff anyway?
            projects.sort(key=lambda p: p["timestamp"], reverse=True)
            app_name = projects[0]["name"]
            
            if logger:
                logger.info(f"Searching most recent project: {app_name}")
        
        key_prefix = f"code:{app_name}"
        results = await query_code_entity(key_prefix, entity_type, name, redis_client)
        
        await redis_client.aclose()
        return results
        
    except Exception as e:
        if logger:
            logger.error(f"Memory search failed: {e}")
        return []

async def async_get_info(app_dir: Optional[Path] = None, logger: Optional[logging.Logger] = None) -> Dict:
    """Get a brain scan of your indexed projects."""
    if not REDIS_PY_AVAILABLE:
        if logger:
            logger.critical("Error: 'redis' library missing - can't check brain health!")
        return {"projects": []}
    
    try:
        redis_client = redis.from_url(REDIS_URL, decode_responses=True)
        await redis_client.ping()
        
        if app_dir:
            # Get info for a specific brain lobe
            app_name = app_dir.resolve().name
            key_prefix = f"code:{app_name}"
            project_info = await get_project_info(key_prefix, redis_client)
            result = {
                "projects": [{"name": app_name, **project_info}]
            }
        else:
            # Check the whole brain
            projects = []
            metadata_keys = await redis_client.keys("code:*:metadata")
            
            for key in metadata_keys:
                app_name = key.split(":")[1]
                key_prefix = f"code:{app_name}"
                project_info = await get_project_info(key_prefix, redis_client)
                projects.append({"name": app_name, **project_info})
            
            # Sort by most recently thought about
            projects.sort(
                key=lambda p: p.get("metadata", {}).get("last_indexed_timestamp", 0), 
                reverse=True
            )
            
            result = {"projects": projects}
        
        await redis_client.aclose()
        return result
        
    except Exception as e:
        if logger:
            logger.error(f"Brain scan failed: {e}")
        return {"projects": []}

async def async_clear_project(app_dir: Path, logger: Optional[logging.Logger] = None) -> bool:
    """Clear memories of a project - like a targeted amnesia treatment."""
    if not REDIS_PY_AVAILABLE:
        if logger:
            logger.critical("Error: 'redis' library missing - nothing to forget!")
        return False
    
    try:
        redis_client = redis.from_url(REDIS_URL, decode_responses=True)
        await redis_client.ping()
        
        app_name = app_dir.resolve().name
        key_prefix = f"code:{app_name}"
        
        # Find all keys for this project
        keys = await redis_client.keys(f"{key_prefix}:*")
        
        if keys:
            if logger:
                logger.info(f"Erasing {len(keys)} memories of {app_name} - was it that bad?")
            await redis_client.delete(*keys)
            await redis_client.aclose()
            return True
        else:
            if logger:
                logger.warning(f"No memories found for {app_name} - nothing to forget!")
            await redis_client.aclose()
            return False
            
    except Exception as e:
        if logger:
            logger.error(f"Memory wipe failed: {e}")
        return False

###############################
# Fancy Terminal Output Stuff #
###############################

def print_entity_details(entity: Dict):
    """Print pretty details about a code entity, with style!"""
    entity_type = entity['entity_type'].upper()
    name = entity['name']
    
    # Colorful headers based on entity type
    if entity_type == "CLASS":
        header = f"🔷 CLASS: {name}"
    elif entity_type == "FUNCTION":
        header = f"🔶 FUNCTION: {name}"
    elif entity_type == "METHOD":
        header = f"🔸 METHOD: {name}"
    elif entity_type == "VARIABLE":
        header = f"💎 VARIABLE: {name}"
    else:
        header = f"{entity_type}: {name}"
    
    print(f"\n{header}")
    print(f"📄 File: {entity['file_path']} (lines {entity['line_start']}-{entity['line_end']})")
    
    if entity['entity_type'] == 'function' or entity['entity_type'] == 'method':
        print(f"📝 Signature: {entity['signature']}")
        if entity.get('parent_class'):
            print(f"👪 Class: {entity['parent_class']}")
    
    elif entity['entity_type'] == 'class':
        if entity.get('bases'):
            bases = ', '.join(entity['bases'])
            print(f"👴 Inherits from: {bases}")
    
    elif entity['entity_type'] == 'variable':
        print(f"💰 Value: {entity.get('value_repr', '?')}")
        if entity.get('parent_class'):
            print(f"🏠 Class: {entity['parent_class']}")
    
    if entity.get('docstring'):
        docstring = entity['docstring'][:200]
        if len(entity['docstring']) > 200:
            docstring += '...'
        print(f"📚 Docstring: {docstring}")
    
    print("➖" * 25)

def print_project_info(info: Dict):
    """Print a fancy report about your brain health."""
    for project in info.get("projects", []):
        print(f"\n🧠 Project: {project['name']}")
        
        metadata = project.get("metadata", {})
        if metadata:
            print(f"🕒 Last indexed: {metadata.get('last_indexed_at', 'never')}")
            
        counts = project.get("counts", {})
        if counts:
            print(f"📁 Files: {counts.get('files', 0)}")
            print(f"🔷 Classes: {counts.get('class', 0)}")
            print(f"🔶 Functions: {counts.get('function', 0)}")
            print(f"🔸 Methods: {counts.get('method', 0)}")
            print(f"💎 Variables: {counts.get('variable', 0)}")
        
        print("➖" * 25)

##############################
# Command Line Shenanigans #
##############################

async def main():
    parser = argparse.ArgumentParser(
        description="🧠 AI's External Brain Cells 🧠",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  brain_cells remember ~/myproject     # Index your entire codebase
  brain_cells refresh app/models.py     # Update specific files
  brain_cells recall class User         # Find all Users in your code
  brain_cells status                   # Check what's in memory
  brain_cells forget                   # Clear everything (Monday morning mode)
        """
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Command to execute")
    
    # Remember command (index)
    remember_parser = subparsers.add_parser("remember", help="Store code in memory")
    remember_parser.add_argument("path", type=str, nargs="?", default=".", 
                            help="Path to the app directory (default: current directory)")
    
    # Refresh command (update)
    refresh_parser = subparsers.add_parser("refresh", help="Update specific files in memory")
    refresh_parser.add_argument("files", type=str, help="Comma-separated list of files to update")
    refresh_parser.add_argument("--project", type=str, help="Project directory (default: current directory)")
    
    # Recall command (query)
    recall_parser = subparsers.add_parser("recall", help="Search for code in memory")
    recall_parser.add_argument("entity_type", choices=["class", "function", "method", "variable"], 
                             help="Type of entity to search for")
    recall_parser.add_argument("name", type=str, nargs="?", help="Name to search for (optional)")
    recall_parser.add_argument("--project", type=str, help="Project to search in (default: most recent)")
    
    # Status command (info)
    status_parser = subparsers.add_parser("status", help="Check what's in memory")
    status_parser.add_argument("--project", type=str, help="Project to check (default: all)")
    
    # Forget command (clear)
    forget_parser = subparsers.add_parser("forget", help="Clear indexed data")
    forget_parser.add_argument("--project", type=str, default=".",
                            help="Project to forget (default: current directory)")
    
    args = parser.parse_args()
    
    # Setup brain activity logging
    log_dir = Path.home() / ".brain_cells" / "logs"
    logger = setup_logging(log_dir)
    
    if args.command == "remember":
        app_dir = Path(args.path).resolve()
        logger.info(f"Memorizing project: {app_dir}")
        
        print(f"🧠 Creating memories of {app_dir.name}...")
        success = await async_index_project(app_dir, None, logger)
        
        if success:
            print(f"✅ Successfully memorized {app_dir.name}! My brain feels bigger!")
        else:
            print(f"❌ Failed to memorize {app_dir.name} - I might have memory issues!")
            sys.exit(1)
    
    elif args.command == "refresh":
        project_dir = Path(args.project).resolve() if args.project else Path(".").resolve()
        files = args.files.split(",")
        logger.info(f"Refreshing memory of {len(files)} files in {project_dir}")
        
        print(f"🔄 Updating memories of specific files in {project_dir.name}...")
        success = await async_index_project(project_dir, files, logger)
        
        if success:
            print(f"✅ Successfully updated memories of {len(files)} files! I feel refreshed!")
        else:
            print(f"❌ Failed to update memories - I'm still living in the past!")
            sys.exit(1)
    
    elif args.command == "recall":
        project_dir = Path(args.project).resolve() if args.project else None
        entity_type = args.entity_type
        name = args.name
        
        print(f"🔍 Searching memory for {entity_type}" + (f" named '{name}'" if name else "s") + "...")
        results = await async_query(entity_type, name, project_dir, logger)
        
        if results:
            print(f"🎉 Found {len(results)} {entity_type}(s)" + 
                 (f" named '{name}'" if name else "") + "!")
            for entity in results:
                print_entity_details(entity)
        else:
            print(f"🤔 No {entity_type}(s) found" + 
                 (f" with name '{name}'" if name else "") + " - are you sure it exists?")
            sys.exit(1)
    
    elif args.command == "status":
        project_dir = Path(args.project).resolve() if args.project else None
        
        print("🔬 Scanning brain cells...")
        info = await async_get_info(project_dir, logger)
        
        if info["projects"]:
            print("📊 Brain scan results:")
            print_project_info(info)
        else:
            print("😴 No projects in memory - my mind is a blank slate!")
            sys.exit(1)
    
    elif args.command == "forget":
        project_dir = Path(args.project).resolve()
        logger.info(f"Forgetting project: {project_dir}")
        
        print(f"🗑️ Clearing memories of {project_dir.name}...")
        success = await async_clear_project(project_dir, logger)
        
        if success:
            print(f"✅ Successfully forgot {project_dir.name}. What were we talking about again?")
        else:
            print(f"❌ Failed to forget or nothing to forget about {project_dir.name}. Some memories are permanent!")
            sys.exit(1)
    
    else:
        parser.print_help()
        print("\n🤔 Not sure what you want me to do with my brain cells...")

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n🛑 Brain activity interrupted! Shutting down neurons...")
        sys.exit(0)
    except Exception as e:
        print(f"🔥 BRAIN MALFUNCTION: {e}", file=sys.stderr)
        sys.exit(1)
