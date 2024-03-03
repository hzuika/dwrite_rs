use windows::{
    core::w,
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::{
            Direct2D::{
                Common::{D2D1_COLOR_F, D2D_RECT_F, D2D_SIZE_U},
                D2D1CreateFactory, ID2D1Factory, D2D1_DRAW_TEXT_OPTIONS_NONE,
                D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES,
                D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES,
                D2D1_WINDOW_STATE_OCCLUDED,
            },
            DirectWrite::{
                DWriteCreateFactory, IDWriteFactory, DWRITE_FACTORY_TYPE_SHARED,
                DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_REGULAR,
                DWRITE_MEASURING_MODE_NATURAL, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
                DWRITE_TEXT_ALIGNMENT_CENTER,
            },
            Gdi::{GetDC, GetDeviceCaps, ReleaseDC, ValidateRect, LOGPIXELSX, LOGPIXELSY},
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW,
            LoadCursorW, PostQuitMessage, RegisterClassW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            IDC_ARROW, MSG, WINDOW_EX_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW,
            WS_VISIBLE,
        },
    },
};

fn get_dpi() -> (f32, f32) {
    unsafe {
        let screen = GetDC(None);
        let dpi_scale_x = GetDeviceCaps(screen, LOGPIXELSX) as f32 / 96.0;
        let dpi_scale_y = GetDeviceCaps(screen, LOGPIXELSY) as f32 / 96.0;
        ReleaseDC(None, screen);
        (dpi_scale_x, dpi_scale_y)
    }
}

// https://github.com/microsoft/Windows-classic-samples/blob/ac06e54a15e9a62443e400fffff190fb978ea586/Samples/Win7Samples/multimedia/DirectWrite/HelloWorld/SimpleText.cpp
fn main() -> anyhow::Result<()> {
    unsafe {
        let (dpi_scale_x, dpi_scale_y) = get_dpi();

        // ウィンドウクラスの作成．
        let instance = GetModuleHandleW(None)?;
        assert!(instance.0 != 0);

        let window_class = w!("window");

        {
            // Register window class.
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            assert!(atom != 0);
        }

        // Create window.
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );
        assert!(hwnd.0 > 0);

        // Create device independent resources.
        let d2d_factory: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
        let dwrite_factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
        let text_format = dwrite_factory.CreateTextFormat(
            w!("Gabriola"),
            None,
            DWRITE_FONT_WEIGHT_REGULAR,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            72.0,
            w!("en-us"),
        )?;
        text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?;
        text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;

        // Create device resources.
        let mut rc = Default::default();
        GetClientRect(hwnd, &mut rc)?;
        let size = D2D_SIZE_U {
            width: (rc.right - rc.left).try_into()?,
            height: (rc.bottom - rc.top).try_into()?,
        };
        let render_target = d2d_factory.CreateHwndRenderTarget(
            &D2D1_RENDER_TARGET_PROPERTIES::default(),
            &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: size,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            },
        )?;
        let black = D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
        let black_brush = render_target.CreateSolidColorBrush(&black, None)?;

        // Draw D2D content.
        if !render_target
            .CheckWindowState()
            .contains(D2D1_WINDOW_STATE_OCCLUDED)
        {
            render_target.BeginDraw();
            render_target.SetTransform(&Matrix3x2::identity());
            render_target.Clear(Some(&white));

            // Draw text.
            let layout_rect = D2D_RECT_F {
                top: rc.top as f32 / dpi_scale_y,
                left: rc.left as f32 / dpi_scale_x,
                right: (rc.right - rc.left) as f32 / dpi_scale_x,
                bottom: (rc.bottom - rc.top) as f32 / dpi_scale_y,
            };

            let text = w!("Hello World using DirectWrite!");
            render_target.DrawText(
                text.as_wide(),
                &text_format,
                &layout_rect,
                &black_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );

            render_target.EndDraw(None, None)?;
        }

        // Message loop.
        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
        }
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
