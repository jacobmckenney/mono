use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // User
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    //TODO: add updated_at to active model trait insert extension
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Name).string())
                    .col(ColumnDef::new(User::Image).string())
                    .to_owned(),
            )
            .await
            .unwrap();
        // Session
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).string().not_null())
                    .col(ColumnDef::new(Session::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::DeviceIp).string())
                    .col(ColumnDef::new(Session::UserAgent).string())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Translation
        manager
            .create_table(
                Table::create()
                    .table(Translation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Translation::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Translation::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Translation::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Translation::Name).string().not_null())
                    .col(ColumnDef::new(Translation::Language).string().not_null())
                    .to_owned(),
            )
            .await?;
        // Book
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Book::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Book::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Book::TranslationId).string().not_null())
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Verses).integer().not_null())
                    .col(ColumnDef::new(Book::ChapterCount).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Book::Table, Book::TranslationId)
                            .to(Translation::Table, Translation::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Chapter
        manager
            .create_table(
                Table::create()
                    .table(Chapter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chapter::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chapter::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Chapter::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Chapter::TranslationId).string().not_null())
                    .col(ColumnDef::new(Chapter::BookId).string().not_null())
                    .col(ColumnDef::new(Chapter::Title).string().not_null())
                    .col(ColumnDef::new(Chapter::VerseCount).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chapter::Table, Chapter::TranslationId)
                            .to(Translation::Table, Translation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chapter::Table, Chapter::BookId)
                            .to(Book::Table, Book::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Verse
        manager
            .create_table(
                Table::create()
                    .table(Verse::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Verse::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Verse::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Verse::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Verse::TranslationId).string().not_null())
                    .col(ColumnDef::new(Verse::BookId).string().not_null())
                    .col(ColumnDef::new(Verse::ChapterId).string().not_null())
                    .col(ColumnDef::new(Verse::Text).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Verse::Table, Verse::TranslationId)
                            .to(Translation::Table, Translation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Verse::Table, Verse::BookId)
                            .to(Book::Table, Book::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Verse::Table, Verse::ChapterId)
                            .to(Chapter::Table, Chapter::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Passage
        manager
            .create_table(
                Table::create()
                    .table(Passage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Passage::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Passage::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Passage::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Passage::FromVerseId).string().not_null())
                    .col(ColumnDef::new(Passage::ToVerseId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Passage::Table, Passage::FromVerseId)
                            .to(Verse::Table, Verse::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Passage::Table, Passage::ToVerseId)
                            .to(Verse::Table, Verse::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Group
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Group::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Group::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Group::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Group::Type).string().not_null())
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::Image).string())
                    .col(ColumnDef::new(Group::Description).string())
                    .to_owned(),
            )
            .await?;
        // Comment
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Comment::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Comment::UserId).string().not_null())
                    .col(ColumnDef::new(Comment::PassageId).string().not_null())
                    .col(ColumnDef::new(Comment::Content).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::PassageId)
                            .to(Passage::Table, Passage::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // MessageThread
        manager
            .create_table(
                Table::create()
                    .table(MessageThread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageThread::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MessageThread::Name).string().not_null())
                    .col(
                        ColumnDef::new(MessageThread::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageThread::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MessageThread::GroupId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(MessageThread::Table, MessageThread::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Message
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Message::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Message::UserId).string().not_null())
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // MessageToMessageThread
        manager
            .create_table(
                Table::create()
                    .table(MessageToMessageThread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageToMessageThread::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MessageToMessageThread::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageToMessageThread::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageToMessageThread::MessageId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageToMessageThread::MessageThreadId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MessageToMessageThread::Table,
                                MessageToMessageThread::MessageId,
                            )
                            .to(Message::Table, Message::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MessageToMessageThread::Table,
                                MessageToMessageThread::MessageThreadId,
                            )
                            .to(MessageThread::Table, MessageThread::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // MessageThreadToGroup
        manager
            .create_table(
                Table::create()
                    .table(MessageThreadToGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageThreadToGroup::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MessageThreadToGroup::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageThreadToGroup::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageThreadToGroup::MessageThreadId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MessageThreadToGroup::GroupId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MessageThreadToGroup::Table,
                                MessageThreadToGroup::MessageThreadId,
                            )
                            .to(MessageThread::Table, MessageThread::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MessageThreadToGroup::Table, MessageThreadToGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // GroupMembership
        manager
            .create_table(
                Table::create()
                    .table(GroupMembership::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GroupMembership::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GroupMembership::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GroupMembership::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GroupMembership::GroupId).string().not_null())
                    .col(ColumnDef::new(GroupMembership::UserId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupMembership::Table, GroupMembership::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupMembership::Table, GroupMembership::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // Prayer
        manager
            .create_table(
                Table::create()
                    .table(Prayer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Prayer::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Prayer::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Prayer::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Prayer::UserId).string().not_null())
                    .col(ColumnDef::new(Prayer::Title).string().not_null())
                    .col(ColumnDef::new(Prayer::Content).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Prayer::Table, Prayer::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // PrayerRequest
        manager
            .create_table(
                Table::create()
                    .table(PrayerRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PrayerRequest::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequest::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequest::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PrayerRequest::UserId).string().not_null())
                    .col(ColumnDef::new(PrayerRequest::Title).string().not_null())
                    .col(ColumnDef::new(PrayerRequest::Content).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PrayerRequest::Table, PrayerRequest::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // PassageToPrayerRequest
        manager
            .create_table(
                Table::create()
                    .table(PassageToPrayerRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PassageToPrayerRequest::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayerRequest::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayerRequest::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayerRequest::PassageId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayerRequest::PrayerRequestId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PassageToPrayerRequest::Table,
                                PassageToPrayerRequest::PassageId,
                            )
                            .to(Passage::Table, Passage::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PassageToPrayerRequest::Table,
                                PassageToPrayerRequest::PrayerRequestId,
                            )
                            .to(PrayerRequest::Table, PrayerRequest::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // PassageToPrayer
        manager
            .create_table(
                Table::create()
                    .table(PassageToPrayer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PassageToPrayer::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayer::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayer::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayer::PassageId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PassageToPrayer::PrayerId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PassageToPrayer::Table, PassageToPrayer::PassageId)
                            .to(Passage::Table, Passage::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PassageToPrayer::Table, PassageToPrayer::PrayerId)
                            .to(Prayer::Table, Prayer::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // PrayerRequestToGroup
        manager
            .create_table(
                Table::create()
                    .table(PrayerRequestToGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PrayerRequestToGroup::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequestToGroup::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequestToGroup::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequestToGroup::PrayerRequestId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PrayerRequestToGroup::GroupId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PrayerRequestToGroup::Table,
                                PrayerRequestToGroup::PrayerRequestId,
                            )
                            .to(PrayerRequest::Table, PrayerRequest::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PrayerRequestToGroup::Table, PrayerRequestToGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PrayerRequestToGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PassageToPrayer::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(PassageToPrayerRequest::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(PrayerRequest::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Prayer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GroupMembership::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MessageThreadToGroup::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(MessageToMessageThread::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MessageThread::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Passage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Verse::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Chapter::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Translation::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Email,
    Image,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
    DeviceIp,
    UserAgent,
}

#[derive(DeriveIden)]
enum Translation {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Language,
}

#[derive(DeriveIden)]
enum Book {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    TranslationId,
    Title,
    Verses,
    ChapterCount,
}

#[derive(DeriveIden)]
enum Chapter {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    TranslationId,
    BookId,
    Title,
    VerseCount,
}

#[derive(DeriveIden)]
enum Verse {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    TranslationId,
    BookId,
    ChapterId,
    Text,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    PassageId,
    Content,
}

#[derive(DeriveIden)]
enum Passage {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    FromVerseId,
    ToVerseId,
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Type,
    Name,
    Image,
    Description,
}

#[derive(DeriveIden)]
enum MessageThread {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    GroupId,
}

#[derive(DeriveIden)]
enum MessageToMessageThread {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    MessageId,
    MessageThreadId,
}

#[derive(DeriveIden)]
enum MessageThreadToGroup {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    MessageThreadId,
    GroupId,
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    Content,
}

#[derive(DeriveIden)]
enum GroupMembership {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    GroupId,
    UserId,
}

#[derive(DeriveIden)]
enum Prayer {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    Title,
    Content,
}

#[derive(DeriveIden)]
enum PrayerRequest {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    Title,
    Content,
}

#[derive(DeriveIden)]
enum PassageToPrayerRequest {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    PassageId,
    PrayerRequestId,
}

#[derive(DeriveIden)]
enum PassageToPrayer {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    PassageId,
    PrayerId,
}

#[derive(DeriveIden)]
enum PrayerRequestToGroup {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    PrayerRequestId,
    GroupId,
}
