pub trait Drawable {
    fn get_vertex_buffer(&self) -> &wgpu::Buffer;
    fn get_num_vertices(&self) -> u32;
    fn get_index_buffer(&self) -> Option<&wgpu::Buffer>;
    fn get_num_indices(&self) -> Option<u32>;

    fn draw<'a: 'b, 'b>(&'a self, render_pass: &mut wgpu::RenderPass<'b>) {
        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        if let Some(index_buffer) = self.get_index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.get_num_indices().unwrap(), 0, 0..1);
        } else {
            render_pass.draw(0..self.get_num_vertices(), 0..1);
        }
    }
}
