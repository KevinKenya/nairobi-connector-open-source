def test_canvas_module_exists():
    """Verify _core.canvas module is importable."""
    try:
        import nairobi_os
        assert hasattr(nairobi_os, 'canvas') or hasattr(nairobi_os, '_core')
    except ImportError as e:
        assert False, f"Failed to import nairobi_os: {e}"


def test_canvas_open_exists():
    """Verify nairobi_os.canvas.open attribute exists."""
    try:
        import nairobi_os
        assert hasattr(nairobi_os.canvas, 'open')
    except ImportError as e:
        assert False, f"Failed to import nairobi_os: {e}"


def test_canvas_namespace_class():
    """Verify CanvasNamespace class exposes open method."""
    try:
        import nairobi_os
        assert hasattr(nairobi_os, 'CanvasNamespace')
        canvas_ns = nairobi_os.CanvasNamespace()
        assert hasattr(canvas_ns, 'open')
    except ImportError as e:
        assert False, f"Failed to import nairobi_os: {e}"