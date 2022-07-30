use graphql_client::*;
use uuid::Uuid;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AcceptQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ActivateSpecialItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AdminGiveItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AdminGivePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AdminLoginAs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AdminSendCustomNotif;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AnswerGuildInvite;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AnswerTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct AwardTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct BidOnAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct BuyGigaItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct BuyGigaMatterCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct BuyVendorItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct BuyoutAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CancelAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CancelTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ClaimGigaMatter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CreateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CreateStarter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CreateTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CreateTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct CreateTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DeleteGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DeleteMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DeleteTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DescendCave;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DevRandomItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DevSavePetItemPos;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DrinkPotion;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct EatFoodItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct EditGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct EvolvePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct FleeBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ForceCatchPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ForgetManyPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ForgetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct FormGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GenDevPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GigaCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GigaPortalLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GiveUpCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct HealPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct InviteToGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct LeaveGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct LinkClick;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct Logout;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct NameCaughtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct PetStorageCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct PickUpQuestItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct PlayWithPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct QuestInteraction;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ReportUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct SavePetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ScareAwayPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct SellAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct SendWildBattleSequence;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct SetGuildRole;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct StabilizePlasma;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct StartCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct StartGuildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct StartNpcBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct StartWildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct SwapDeadPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct TakeIceCream;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct Teleport;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ThrowItemAtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct TryToFriendCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct TurnInQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdateMainPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdateMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdateMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdatePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdatePetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UpdateRep;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct UsernameChangeCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DevAllItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DevGetPetItemMap;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct DevPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetAuctionItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetBondLeaderboard;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetDailyQuests;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetGigaStoreItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetGuildInvites;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetInventory;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetLinkAnalytics;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetNumUnreadNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetPetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetPetLog;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetPetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetStarterAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetTradeBoxes;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetTradeOffers;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetTrophies;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct GetVendorItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct ItemNameSearch;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries.graphql"
)]
pub struct Me2;