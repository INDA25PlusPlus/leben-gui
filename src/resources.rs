use std::fmt::Debug;
use ggez::context::Context;
use ggez::GameResult;
use ggez::graphics::Image;
use rsoderh_chess::Piece;

pub const WHITE_PAWN_IMAGE: &str = "/pieces/Chess_plt45.png";
pub const BLACK_PAWN_IMAGE: &str = "/pieces/Chess_pdt45.png";
pub const WHITE_KNIGHT_IMAGE: &str = "/pieces/Chess_nlt45.png";
pub const BLACK_KNIGHT_IMAGE: &str = "/pieces/Chess_ndt45.png";
pub const WHITE_BISHOP_IMAGE: &str = "/pieces/Chess_blt45.png";
pub const BLACK_BISHOP_IMAGE: &str = "/pieces/Chess_bdt45.png";
pub const WHITE_ROOK_IMAGE: &str = "/pieces/Chess_rlt45.png";
pub const BLACK_ROOK_IMAGE: &str = "/pieces/Chess_rdt45.png";
pub const WHITE_QUEEN_IMAGE: &str = "/pieces/Chess_qlt45.png";
pub const BLACK_QUEEN_IMAGE: &str = "/pieces/Chess_qdt45.png";
pub const WHITE_KING_IMAGE: &str = "/pieces/Chess_klt45.png";
pub const BLACK_KING_IMAGE: &str = "/pieces/Chess_kdt45.png";

/// Collection of various resources required for the application
#[derive(Clone, Debug)]
pub struct Resources {
    pub images: ImageResources,
}

/// Contains a resource with its associated file path
#[derive(Clone, Debug)]
pub struct ResourceEntry<T: Clone + Debug> {
    pub path: &'static str,
    pub resource: T,
}

/// Collection of all the images required for the application
#[derive(Clone, Debug)]
pub struct ImageResources {
    pub white_pawn: ResourceEntry<Image>,
    pub black_pawn: ResourceEntry<Image>,
    pub white_knight: ResourceEntry<Image>,
    pub black_knight: ResourceEntry<Image>,
    pub white_bishop: ResourceEntry<Image>,
    pub black_bishop: ResourceEntry<Image>,
    pub white_rook: ResourceEntry<Image>,
    pub black_rook: ResourceEntry<Image>,
    pub white_queen: ResourceEntry<Image>,
    pub black_queen: ResourceEntry<Image>,
    pub white_king: ResourceEntry<Image>,
    pub black_king: ResourceEntry<Image>,
}

impl ImageResources {
    /// returns: Which image resource corresponds to the given piece
    pub fn get_piece(&self, piece: Piece) -> &ResourceEntry<Image> {
        use rsoderh_chess::Color::*;
        use rsoderh_chess::PieceKind::*;
        match (piece.color, piece.kind) {
            (White, Pawn) => &self.white_pawn,
            (Black, Pawn) => &self.black_pawn,
            (White, Knight) => &self.white_knight,
            (Black, Knight) => &self.black_knight,
            (White, Bishop) => &self.white_bishop,
            (Black, Bishop) => &self.black_bishop,
            (White, Rook) => &self.white_rook,
            (Black, Rook) => &self.black_rook,
            (White, Queen) => &self.white_queen,
            (Black, Queen) => &self.black_queen,
            (White, King) => &self.white_king,
            (Black, King) => &self.black_king,
        }
    }
}

/// Create an image resource given its file path
fn create_image_resource(ctx: &mut Context,
                         path: &'static str) -> GameResult<ResourceEntry<Image>>
{
    Ok(ResourceEntry {
        path,
        resource: Image::from_path(ctx, path)?,
    })
}

impl Resources {
    /// Instantiate a `Resources` object, loading all required resources
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        Ok(Resources {
            images: ImageResources {
                white_pawn: create_image_resource(ctx, WHITE_PAWN_IMAGE)?,
                black_pawn: create_image_resource(ctx, BLACK_PAWN_IMAGE)?,
                white_knight: create_image_resource(ctx, WHITE_KNIGHT_IMAGE)?,
                black_knight: create_image_resource(ctx, BLACK_KNIGHT_IMAGE)?,
                white_bishop: create_image_resource(ctx, WHITE_BISHOP_IMAGE)?,
                black_bishop: create_image_resource(ctx, BLACK_BISHOP_IMAGE)?,
                white_rook: create_image_resource(ctx, WHITE_ROOK_IMAGE)?,
                black_rook: create_image_resource(ctx, BLACK_ROOK_IMAGE)?,
                white_queen: create_image_resource(ctx, WHITE_QUEEN_IMAGE)?,
                black_queen: create_image_resource(ctx, BLACK_QUEEN_IMAGE)?,
                white_king: create_image_resource(ctx, WHITE_KING_IMAGE)?,
                black_king: create_image_resource(ctx, BLACK_KING_IMAGE)?,
            }
        })
    }
}
