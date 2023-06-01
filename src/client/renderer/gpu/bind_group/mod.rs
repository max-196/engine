pub struct BindGroup {
    pub group: wgpu::BindGroup,
    pub layout: Layout,
}

impl BindGroup {
    pub fn new(
        device: &wgpu::Device,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
        group_entries: &[wgpu::BindGroupEntry],
        label: &str) -> Self {

        let layout = Layout::new(
            device,
            layout_entries,
            &(label.to_owned() + "Bind Group Layout"),
        );

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout.0,
                entries: group_entries,
                label: Some(&(label.to_owned() + "Bind Group")),
            }
        );

        Self {
            group,
            layout,
        }
    }

    pub fn replace_group(&mut self, device: &wgpu::Device, entries: &[wgpu::BindGroupEntry], label: &str) {
        self.group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.layout(),
            entries,
            label: Some(&(label.to_owned() + "Bind Group")),
        });
    }

    pub fn with_layout(device: &wgpu::Device,
        layout: Layout,
        group_entries: &[wgpu::BindGroupEntry],
        label: &str) -> Self {

        let group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout.0,
                entries: group_entries,
                label: Some(&(label.to_owned() + "Bind Group"))
            }
        );

        Self { group, layout }
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout.0
    }
}

pub struct Layout(pub wgpu::BindGroupLayout);

impl Layout {
    pub fn new(
        device: &wgpu::Device,
        entries: &[wgpu::BindGroupLayoutEntry],
        label: &str,
    ) -> Self {
        Self(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries,
            label: Some(label),
        }))
    }
}